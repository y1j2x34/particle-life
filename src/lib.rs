#![allow(dead_code)]
#![allow(non_snake_case)]

extern crate wasm_bindgen;

use js_sys::{Object, Reflect, Math};
use wasm_bindgen::{prelude::*, __rt::WasmRefCell};


#[derive(Clone)]
pub struct Atom {
    x: f64,
    y: f64,
    vx: f64,
    vy: f64,
    color: String
}

pub struct ParticleWord {
    pub atoms: Vec<Atom>,
    pub width: i32,
    pub height: i32
}

type Rule = Object;

impl ParticleWord {

    pub fn new(width: i32, height: i32) -> Self {
        ParticleWord {
            atoms: Vec::new(),
            width,
            height
        }
    }

    fn apply_rules(& mut self, rule: &Rule) {
        let size = self.atoms.len();

        let mut i = 0;
        let mut j = 0;

        while i < size {
            
            let atom_i_option = self.atoms.get(i);

            if let None = atom_i_option {
                continue;
            }
            
            let mut atom_i = atom_i_option.unwrap().clone();

            let mut fx: f64 = 0.0;
            let mut fy: f64 = 0.0;


            while j < size {
                if i == j {
                    continue;
                }
                let atom_j_option = self.atoms.get(j);

                if let None = atom_j_option {
                    continue;
                }

                let atom_j = atom_j_option.unwrap();

                

                let ri = Reflect::get(rule, &JsValue::from_str(atom_i.color.as_str())) ;
                if let Err(_) = ri {
                    continue;
                }
                

                let rj_opt = Reflect::get(&ri.unwrap(), &JsValue::from_str(atom_j.color.as_str()));

                if let Err(_) = rj_opt {
                    continue;
                }
                let og = rj_opt.unwrap().as_f64();

                if let None = og {
                    continue;
                }
                let g = og.unwrap();

                let dx = atom_i.x - atom_j.x;
                let dy = atom_i.y - atom_j.y;

                if dx == 0.0 || dy == 0.0 {
                    continue;
                }

                let d = dx * dx + dy * dy;

                if d < 6400.0 {
                    let f = g / d.sqrt();
                    fx = fx + f * dx;
                    fy = fy + f * dy;
                }
                j = j + 1
            } // j
            
            atom_i.vx = (atom_i.vx + fx) * 0.5;
            atom_i.vy = (atom_i.vy + fy) * 0.5;

            atom_i.x = atom_i.x + atom_i.vx;
            atom_i.y = atom_i.y + atom_i.vy;

            if atom_i.x <= 0.0 {
                atom_i.vx = -atom_i.vx;
                atom_i.x = 0.0;
            }

            let width = self.width as f64;
            let height = self.height as f64;

            if atom_i.x >= width {
                atom_i.vx = - atom_i.vx;
                atom_i.x = width;
            }

            if atom_i.y <= 0.0 {
                atom_i.vy = -atom_i.vy;
                atom_i.y = 0.0;
            }

            if atom_i.y >= height {
                atom_i.vy = -atom_i.vy;
                atom_i.y = height;
            }
            self.atoms[i] = atom_i;

            i = i + 1;
        } // i
    }

    fn random(& self) -> f64 {
        Math::random() * ((self.height - 100) * 50) as f64
    }
    
}


#[export_name = "ParticleWord_new"]
pub extern "C" fn __wasm_bindgen_generated_ParticleWord_new(width: i32, height: i32) -> u32 {
    let particle_word = ParticleWord::new(width, height);
    Box::into_raw(Box::new(WasmRefCell::new(particle_word))) as u32
}

#[export_name = "ParticleWord_apply_rules"]
pub extern "C" fn __wasm_bindgen_generated_ParticleWord_apply_rules(ptr: u32, rule: &Rule) {
    let instance = ptr as *mut WasmRefCell<ParticleWord>;
    wasm_bindgen::__rt::assert_not_null(instance);
    let instance = unsafe { &* instance };

    instance.borrow_mut().apply_rules(rule);
}

#[no_mangle]
pub unsafe extern "C" fn __wbindgen_ParticleWord_free(ptr: u32) {
    let instance = ptr as *mut WasmRefCell<ParticleWord>;
    wasm_bindgen::__rt::assert_not_null(instance);
    (*instance).borrow_mut();
    drop(Box::from_raw(instance));
}
