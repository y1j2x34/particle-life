#![allow(dead_code)]
#![allow(non_snake_case)]

extern crate wasm_bindgen;
extern crate serde_json;

use std::{collections::HashMap};

use js_sys::{ Math };
use web_sys;
// use web_sys::{ CanvasRenderingContext2d };
use wasm_bindgen::{prelude::*, __rt::WasmRefCell};

type Rule = HashMap<String, HashMap<String, f64>>;


#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn error(s: &str);
}
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
    pub width: f64,
    pub height: f64,
    pub rule: Rule,
    context: web_sys::CanvasRenderingContext2d
}


impl ParticleWord {

    pub fn new(width: f64, height: f64, ruleJson: String, context: web_sys::CanvasRenderingContext2d) -> Self {
        let ruleResult: Result<Rule, _> = serde_json::from_str(ruleJson.as_str());
        // let rule: Rule = ruleObject.into_serde().unwrap();
        // let ruleResult: Result<Rule, _> = serde_wasm_bindgen::from_value(ruleObject);

        if let Err(_) = ruleResult {
            error(format!("{:?}", &ruleResult).as_str());
            panic!("error");
        }
        let rule = ruleResult.unwrap();
        log(format!("rule: {:?}", &rule).as_str());

        ParticleWord {
            atoms: Vec::new(),
            width,
            height,
            rule,
            context
        }
    }

    fn apply_rules(& mut self) {
        let rule = &self.rule;

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

                

                // let ri = Reflect::get(rule, &JsValue::from_str(atom_i.color.as_str())) ;
                // if let Err(_) = ri {
                //     continue;
                // }
                let ri = rule.get(atom_i.color.as_str());

                if let None = ri {
                    continue;
                }
                let rj_opt = ri.unwrap().get(atom_j.color.as_str());
                
                if let None = rj_opt {
                    continue;
                }

                // let rj_opt = Reflect::get(&ri.unwrap(), &JsValue::from_str(atom_j.color.as_str()));

                // if let Err(_) = rj_opt {
                //     continue;
                // }

                // let og = rj_opt.unwrap().as_f64();

                // if let None = og {
                //     continue;
                // }
                // let g = og.unwrap();

                let g = rj_opt.unwrap();

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

            let width = self.width;
            let height = self.height;

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

    fn prepare(&mut self, atomsCount: i32) {
        let colors = self.rule.keys().clone();

        for color in colors {
            for _ in 0..atomsCount {
                self.atoms.push(Atom {
                    x: self.random() * self.width,
                    y: self.random() * self.height,
                    vx: 0.0,
                    vy: 0.0,
                    color: color.to_string()
                });
            }
        }
    }

    fn render(&self) {
        self.context.save();
        self.context.set_fill_style(&JsValue::from_str("black"));
        self.context.fill_rect(0.0, 0.0, self.width, self.height);
        let atoms = self.atoms.clone();
        for atom in atoms.into_iter() {
            self.context.set_fill_style(&JsValue::from_str(atom.color.as_str()));
            self.context.fill_rect(atom.x, atom.y, 2.0, 2.0);
        }
        self.context.restore();
    }

    fn random(& self) -> f64 {
        Math::random() * ((self.height - 100.0) * 50.0) as f64
    }
    
}


#[wasm_bindgen]
pub fn new_ParticleWord(width: f64, height: f64, ruleJson: String, context: web_sys::CanvasRenderingContext2d) -> u32 {
    log(format!("rule json: {:?}, width: {}, height: {} ", ruleJson, width, height).as_str());
    let mut particle_word = ParticleWord::new(width, height, ruleJson, context);
    particle_word.prepare(500);
    Box::into_raw(Box::new(WasmRefCell::new(particle_word))) as u32
}

#[wasm_bindgen]
pub extern "C" fn apply_rules(ptr: u32) {
    let instance = ptr as *mut WasmRefCell<ParticleWord>;
    wasm_bindgen::__rt::assert_not_null(instance);
    let instance = unsafe { &* instance };

    instance.borrow_mut().apply_rules();
}

#[wasm_bindgen]
pub fn render(ptr: u32) {
    let instance = ptr as *mut WasmRefCell<ParticleWord>;
    wasm_bindgen::__rt::assert_not_null(instance);
    let instance = unsafe { &* instance };
    instance.borrow().render();
}

#[wasm_bindgen]
pub fn delete_ParticleWord(ptr: u32) {
    let instance = ptr as *mut WasmRefCell<ParticleWord>;
    wasm_bindgen::__rt::assert_not_null(instance);
    unsafe {
        (*instance).borrow_mut();
        drop(Box::from_raw(instance));
    }
}
