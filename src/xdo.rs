use std::ptr::{null, null_mut};

use libxdo_sys::{
    xdo_clear_active_modifiers, xdo_click_window, xdo_free, xdo_get_active_modifiers,
    xdo_get_mouse_location, xdo_get_viewport_dimensions, xdo_mouse_down, xdo_mouse_up,
    xdo_move_mouse, xdo_move_mouse_relative, xdo_new, xdo_set_active_modifiers, Struct_xdo,
    CURRENTWINDOW,
};

pub struct XDO {
    pub raw: *mut Struct_xdo,
}

impl XDO {
    pub fn new() -> Result<Self, ()> {
        let raw = unsafe {
            let xdo = xdo_new(null());
            xdo
        };

        Ok(Self { raw })
    }

    pub fn click(&mut self, bt: i32, lp: u32, sleep: u64) {
        let mut mods = null_mut();
        let mut modct = 0;

        unsafe {
            xdo_get_active_modifiers(self.raw, &mut mods, &mut modct);
            xdo_clear_active_modifiers(self.raw, CURRENTWINDOW, mods, modct);
        }

        for i in 1..=lp {
            unsafe {
                xdo_click_window(self.raw, CURRENTWINDOW, bt);
            }

            if i < lp {
                std::thread::sleep(std::time::Duration::from_millis(sleep))
            }
        }

        unsafe {
            xdo_set_active_modifiers(self.raw, CURRENTWINDOW, mods, modct);
        }
    }

    pub fn postition(&mut self) -> (i32, i32) {
        let mut x = 0i32;
        let mut y = 0i32;
        let mut s = 0i32;

        unsafe {
            xdo_get_mouse_location(self.raw, &mut x, &mut y, &mut s);
        }

        (x, y)
    }

    pub fn viewport(&mut self) -> (u32, u32) {
        let mut w = 0u32;
        let mut h = 0u32;

        unsafe {
            xdo_get_viewport_dimensions(self.raw, &mut w, &mut h, 0);
            (w, h)
        }
    }

    pub fn relative_move(&self, x: i32, y: i32) {
        unsafe {
            xdo_move_mouse_relative(self.raw, x, y);
        }
    }

    pub fn centralize(&mut self) {
        let dm = self.viewport();

        unsafe {
            xdo_move_mouse(self.raw, (dm.0 / 2) as i32, (dm.1 / 2) as i32, 0);
        }
    }

    pub(crate) fn mouse_up(&self) {
        unsafe {
            xdo_mouse_up(self.raw, CURRENTWINDOW, 1);
        }
    }

    pub(crate) fn mouse_down(&self) {
        unsafe {
            xdo_mouse_down(self.raw, CURRENTWINDOW, 1);
        }
    }
}

impl Drop for XDO {
    fn drop(&mut self) {
        unsafe {
            xdo_free(self.raw);
        }
    }
}
