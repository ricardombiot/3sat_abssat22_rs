
use crate::abssat::gpath::gpow::GPow;
use crate::abssat::sat_machine::{SatMachine, init_timeline};

impl SatMachine {
    pub fn new(n_vars : i32) -> SatMachine {
        let current_step = 0;
        let timeline  = init_timeline();
        let is_valid = true;
        let is_close = false;
        let mut machine = SatMachine { 
                timeline, 
                current_step, 
                n_vars , 
                is_valid, 
                is_close
        };
        
        machine.init();
        return machine;
    }

    fn init(&mut self) {
        self.create_first_var();
        self.create_vars();
        self.fusion_vars_step();
    }

    fn create_first_var(&mut self){
        let mut gpath_init0 = GPow::new();
        gpath_init0.do_up((0,0));
        gpath_init0.do_up((1,1));
        let mut gpath_init1 = GPow::new();
        gpath_init1.do_up((0,1));
        gpath_init1.do_up((1,0));

        self.timeline[1] = Some(gpath_init0);
        self.timeline[0] = Some(gpath_init1);
        self.current_step += 2;
    }

    fn create_vars(&mut self){
        for _i in 1..self.n_vars {
            let gpath0 = self.timeline[0].as_ref().unwrap();
            let mut gpath0_to_0 = gpath0.clone();
            let mut gpath0_to_1 = gpath0.clone();
     
            let gpath1 = self.timeline[1].as_ref().unwrap();
            let mut gpath1_to_0 = gpath1.clone();
            let mut gpath1_to_1 = gpath1.clone();
     
            gpath0_to_0.do_up((self.current_step,0));
            gpath1_to_0.do_up((self.current_step,0));
     
            let mut gpath_on0joined = gpath0_to_0;
            gpath_on0joined.do_join(&gpath1_to_0);
     
            gpath0_to_1.do_up((self.current_step,1));
            gpath1_to_1.do_up((self.current_step,1));
     
            let mut gpath_on1joined = gpath0_to_1;
            gpath_on1joined.do_join(&gpath1_to_1);
     
            self.current_step += 1;
            // Negados
            gpath_on0joined.do_up((self.current_step,1));
            gpath_on1joined.do_up((self.current_step,0));
     
            self.timeline[1] = Some(gpath_on0joined);
            self.timeline[0] = Some(gpath_on1joined);
            self.current_step += 1;
        }
    }

    fn fusion_vars_step(&mut self){
        let gpath0 = self.timeline[0].as_ref().unwrap();
        let mut gpath0_to_fusion = gpath0.clone();
    
        let gpath1 = self.timeline[1].as_ref().unwrap();
        let mut gpath1_to_fusion = gpath1.clone();
    
        gpath0_to_fusion.do_up((self.current_step,0));
        gpath1_to_fusion.do_up((self.current_step,0));
    
        let mut gpath_fusion = gpath0_to_fusion;
        gpath_fusion.do_join(&gpath1_to_fusion);


        self.timeline = init_timeline();
        self.timeline[0] = Some(gpath_fusion);
    
        self.current_step += 1;
    }
}