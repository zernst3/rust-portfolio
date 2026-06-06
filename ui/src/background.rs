//! Animated water cinemagraph background, authored entirely in Rust via
//! `web-sys` (Rust bindings to the browser's WebGL API).
//!
//! It renders the NYC blue-hour photo into `#background-canvas`: the photo is
//! static except the East River, which ripples (full motion close to the
//! viewer, tapering to a faint floor at the far shoreline so the skyline never
//! warps). Hovering the page lifts the water sparkle slightly.
//!
//! The only non-Rust artifacts are irreducible on the web: the GLSL shader
//! source below (WebGL's shading language) and the wasm-bindgen glue Cargo
//! generates for every web build. All orchestration — context setup, buffers,
//! texture upload, the requestAnimationFrame loop, and event listeners — is
//! Rust. Compiled only for wasm32; a no-op stub exists for native builds.
//!
//! Robustness: setup is fallible end-to-end and failures are swallowed, so it
//! can never panic into the page. If WebGL is unavailable (or the image fails
//! to load) it falls back to the still photo as a CSS background. The rAF loop
//! is naturally throttled to ~0 by the browser while the tab is hidden, and it
//! honors `prefers-reduced-motion` by drawing a single static frame.

use std::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::closure::Closure;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{HtmlCanvasElement, HtmlImageElement, WebGlRenderingContext as Gl, WebGlShader};

const IMG_URL: &str = "/static/images/nyc.jpg";

const VERT: &str = "attribute vec2 p;void main(){gl_Position=vec4(p,0.0,1.0);}";

const FRAG: &str = r#"
precision highp float;
uniform vec2 R; uniform vec2 IMG; uniform float T; uniform float BOOST;
uniform sampler2D tex;
void main(){
  vec2 uv=gl_FragCoord.xy/R;
  float ca=R.x/R.y, ia=IMG.x/IMG.y;
  vec2 cuv=uv;
  if(ca>ia){cuv.y=(uv.y-0.5)*(ia/ca)+0.5;} else {cuv.x=(uv.x-0.5)*(ca/ia)+0.5;}
  vec2 ccuv=clamp(cuv,0.0,1.0);
  float wl=0.42;                                       // far-shore waterline
  float cityMask=smoothstep(wl+0.01, wl-0.01, cuv.y);  // 1 below waterline, 0 above
  float tnorm=clamp((wl-cuv.y)/wl, 0.0, 1.0);          // 0 at horizon -> 1 at bottom
  float ramp=tnorm;                                     // linear taper keeps mid-distance alive
  float amp=0.0016 + 0.014*ramp;                        // tiny floor far -> big up close
  vec2 wuv=ccuv;
  float r1=sin(cuv.x*52.0+T*1.6+sin(cuv.y*38.0+T*1.1)*2.0);
  float r2=sin(cuv.y*95.0-T*2.2);
  wuv.y+=(r1*0.6+r2*0.4)*amp;
  wuv.x+=sin(cuv.y*70.0+T*1.3)*amp*0.45;
  vec2 fgUV=mix(ccuv,wuv,cityMask);
  vec3 col=texture2D(tex,clamp(fgUV,0.0,1.0)).rgb;
  col+=col*max(0.0,sin(cuv.x*140.0+cuv.y*60.0+T*3.2))*cityMask*ramp*(0.10+0.10*BOOST);
  gl_FragColor=vec4(col,1.0);
}
"#;

/// Mount the background once. Safe to call on every render; subsequent calls
/// are no-ops. Never panics: any setup failure leaves the canvas's CSS color.
pub fn mount() {
    thread_local! { static DONE: RefCell<bool> = const { RefCell::new(false) }; }
    let already = DONE.with(|d| {
        let v = *d.borrow();
        *d.borrow_mut() = true;
        v
    });
    if already {
        return;
    }
    let _ = try_mount();
}

fn fallback_still(canvas: &HtmlCanvasElement) {
    let style = canvas.style();
    let _ = style.set_property("background-image", &format!("url('{IMG_URL}')"));
    let _ = style.set_property("background-size", "cover");
    let _ = style.set_property("background-position", "center");
}

fn compile(gl: &Gl, kind: u32, src: &str) -> Option<WebGlShader> {
    let shader = gl.create_shader(kind)?;
    gl.shader_source(&shader, src);
    gl.compile_shader(&shader);
    Some(shader)
}

fn try_mount() -> Result<(), JsValue> {
    let window = web_sys::window().ok_or(JsValue::NULL)?;
    let document = window.document().ok_or(JsValue::NULL)?;
    let canvas = match document.get_element_by_id("background-canvas") {
        Some(el) => el.dyn_into::<HtmlCanvasElement>()?,
        None => return Ok(()),
    };
    let gl = match canvas.get_context("webgl")? {
        Some(ctx) => ctx.dyn_into::<Gl>()?,
        None => {
            fallback_still(&canvas);
            return Ok(());
        }
    };

    // Compile + link the fullscreen-quad program.
    let vs = compile(&gl, Gl::VERTEX_SHADER, VERT).ok_or(JsValue::NULL)?;
    let fs = compile(&gl, Gl::FRAGMENT_SHADER, FRAG).ok_or(JsValue::NULL)?;
    let program = gl.create_program().ok_or(JsValue::NULL)?;
    gl.attach_shader(&program, &vs);
    gl.attach_shader(&program, &fs);
    gl.link_program(&program);
    if !gl
        .get_program_parameter(&program, Gl::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        fallback_still(&canvas);
        return Ok(());
    }
    gl.use_program(Some(&program));

    // Fullscreen triangle covering the viewport.
    let buffer = gl.create_buffer().ok_or(JsValue::NULL)?;
    gl.bind_buffer(Gl::ARRAY_BUFFER, Some(&buffer));
    let verts: [f32; 6] = [-1.0, -1.0, 3.0, -1.0, -1.0, 3.0];
    // Copy into a JS typed array (safe; no borrowed view of wasm memory, so this
    // satisfies the workspace's forbid-unsafe lint).
    let array = js_sys::Float32Array::new_with_length(verts.len() as u32);
    array.copy_from(&verts);
    gl.buffer_data_with_array_buffer_view(Gl::ARRAY_BUFFER, &array, Gl::STATIC_DRAW);
    let loc = gl.get_attrib_location(&program, "p") as u32;
    gl.enable_vertex_attrib_array(loc);
    gl.vertex_attrib_pointer_with_i32(loc, 2, Gl::FLOAT, false, 0, 0);

    let u_r = gl.get_uniform_location(&program, "R");
    let u_img = gl.get_uniform_location(&program, "IMG");
    let u_t = gl.get_uniform_location(&program, "T");
    let u_b = gl.get_uniform_location(&program, "BOOST");
    gl.uniform1i(gl.get_uniform_location(&program, "tex").as_ref(), 0);

    let dpr = window.device_pixel_ratio().clamp(1.0, 2.0);

    // Resize: keep the drawing buffer at native pixel density (crisp).
    let resize: Rc<dyn Fn()> = {
        let gl = gl.clone();
        let canvas = canvas.clone();
        let window = window.clone();
        Rc::new(move || {
            let w = window
                .inner_width()
                .ok()
                .and_then(|v| v.as_f64())
                .unwrap_or(1280.0);
            let h = window
                .inner_height()
                .ok()
                .and_then(|v| v.as_f64())
                .unwrap_or(720.0);
            canvas.set_width((w * dpr) as u32);
            canvas.set_height((h * dpr) as u32);
            gl.viewport(0, 0, (w * dpr) as i32, (h * dpr) as i32);
        })
    };
    resize();
    {
        let resize = resize.clone();
        let cb = Closure::wrap(Box::new(move || resize()) as Box<dyn FnMut()>);
        window.add_event_listener_with_callback("resize", cb.as_ref().unchecked_ref())?;
        cb.forget();
    }

    // Hover brightens the water sparkle (desktop pointer).
    let target = Rc::new(RefCell::new(0.0_f64));
    let boost = Rc::new(RefCell::new(0.0_f64));
    if let Some(doc_el) = document.document_element() {
        let t = target.clone();
        let enter = Closure::wrap(Box::new(move || *t.borrow_mut() = 1.0) as Box<dyn FnMut()>);
        doc_el.add_event_listener_with_callback("mouseenter", enter.as_ref().unchecked_ref())?;
        enter.forget();
        let t = target.clone();
        let leave = Closure::wrap(Box::new(move || *t.borrow_mut() = 0.0) as Box<dyn FnMut()>);
        doc_el.add_event_listener_with_callback("mouseleave", leave.as_ref().unchecked_ref())?;
        leave.forget();
    }

    let reduce = window
        .match_media("(prefers-reduced-motion: reduce)")
        .ok()
        .flatten()
        .map(|m| m.matches())
        .unwrap_or(false);

    // Load the photo, upload it as a texture, then start the loop.
    let image = HtmlImageElement::new()?;
    {
        let gl = gl.clone();
        let canvas = canvas.clone();
        let image_for_load = image.clone();
        let onload = Closure::wrap(Box::new(move || {
            let tex = match gl.create_texture() {
                Some(t) => t,
                None => return,
            };
            gl.bind_texture(Gl::TEXTURE_2D, Some(&tex));
            gl.pixel_storei(Gl::UNPACK_FLIP_Y_WEBGL, 1);
            if gl
                .tex_image_2d_with_u32_and_u32_and_image(
                    Gl::TEXTURE_2D,
                    0,
                    Gl::RGB as i32,
                    Gl::RGB,
                    Gl::UNSIGNED_BYTE,
                    &image_for_load,
                )
                .is_err()
            {
                return;
            }
            gl.tex_parameteri(Gl::TEXTURE_2D, Gl::TEXTURE_WRAP_S, Gl::CLAMP_TO_EDGE as i32);
            gl.tex_parameteri(Gl::TEXTURE_2D, Gl::TEXTURE_WRAP_T, Gl::CLAMP_TO_EDGE as i32);
            gl.tex_parameteri(Gl::TEXTURE_2D, Gl::TEXTURE_MIN_FILTER, Gl::LINEAR as i32);
            gl.tex_parameteri(Gl::TEXTURE_2D, Gl::TEXTURE_MAG_FILTER, Gl::LINEAR as i32);

            let iw = image_for_load.natural_width() as f32;
            let ih = image_for_load.natural_height() as f32;

            // One frame's draw call.
            let draw = {
                let gl = gl.clone();
                let canvas = canvas.clone();
                let (u_r, u_img, u_t, u_b) = (u_r.clone(), u_img.clone(), u_t.clone(), u_b.clone());
                let (boost, target) = (boost.clone(), target.clone());
                move |t_sec: f64| {
                    let tgt = *target.borrow();
                    {
                        let mut b = boost.borrow_mut();
                        *b += (tgt - *b) * 0.06;
                    }
                    gl.uniform2f(u_r.as_ref(), canvas.width() as f32, canvas.height() as f32);
                    gl.uniform2f(u_img.as_ref(), iw, ih);
                    gl.uniform1f(u_t.as_ref(), t_sec as f32);
                    gl.uniform1f(u_b.as_ref(), *boost.borrow() as f32);
                    gl.draw_arrays(Gl::TRIANGLES, 0, 3);
                }
            };

            if reduce {
                draw(0.0);
                return;
            }

            // Self-scheduling requestAnimationFrame loop (the canonical web-sys
            // pattern: the closure holds an Rc to itself so it stays alive).
            let f: Rc<RefCell<Option<Closure<dyn FnMut(f64)>>>> = Rc::new(RefCell::new(None));
            let g = f.clone();
            *g.borrow_mut() = Some(Closure::wrap(Box::new(move |ms: f64| {
                draw(ms / 1000.0);
                if let Some(w) = web_sys::window() {
                    if let Some(cb) = f.borrow().as_ref() {
                        let _ = w.request_animation_frame(cb.as_ref().unchecked_ref());
                    }
                }
            }) as Box<dyn FnMut(f64)>));
            if let (Some(w), Some(cb)) = (web_sys::window(), g.borrow().as_ref()) {
                let _ = w.request_animation_frame(cb.as_ref().unchecked_ref());
            }
            // Keep the loop alive for the lifetime of the page.
            std::mem::forget(g);
        }) as Box<dyn FnMut()>);
        image.set_onload(Some(onload.as_ref().unchecked_ref()));
        onload.forget();
    }
    {
        let canvas = canvas.clone();
        let onerror = Closure::wrap(Box::new(move || fallback_still(&canvas)) as Box<dyn FnMut()>);
        image.set_onerror(Some(onerror.as_ref().unchecked_ref()));
        onerror.forget();
    }
    image.set_src(IMG_URL);

    Ok(())
}
