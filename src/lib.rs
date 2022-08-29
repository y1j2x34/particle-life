extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;
use rand;
use serde::{ Serialize, Deserialize };
use js_sys::{ Object, Reflect };

#[derive(Clone, Serialize, Deserialize)]
pub struct Atom {
    x: f64,
    y: f64,
    vx: f64,
    vy: f64,
    color: String
}


#[derive(Serialize, Deserialize)]
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

    fn applyRules(& mut self, rule: &Rule) {
        let size = self.atoms.len();

        let mut i = 0;
        let mut j = 0;

        while i < size {
            let mut fx = 0;
            let mut fy = 0;
            
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
        } // i
    }

    fn random(& self) -> f64 {
        rand::random::<f64>() * ((self.height - 100) * 50) as f64
    }
    
}

#[wasm_bindgen]
pub fn createParticleRule(width: i32, height: i32) {
    ParticleWord::new(width, height)
}