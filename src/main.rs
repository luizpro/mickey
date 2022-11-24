mod xdo;
use serde::{Deserialize, Serialize};
use xdo::XDO;

use clap::Parser;

use anyhow::anyhow;
use anyhow::Result;

#[derive(Parser, Debug)]
pub struct App {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Parser, Debug)]
pub enum Command {
    Click(Click),
    Centralize,
    Move(Move),
    Screenshot,
    Hold,
}

#[derive(Parser, Debug)]
pub struct Click {
    #[clap(short, long, default_value_t = 1)]
    repeat: u32,

    #[clap(short, long, default_value_t = 5)]
    sleep: u64,

    #[clap(subcommand)]
    buton: MouseButton,
}

impl Click {
    fn fire(&self, mut xdo: XDO) -> Result<()> {
        xdo.click(self.buton as i32, self.repeat, self.sleep);

        State::reset()?;
        Ok(())
    }
}

#[derive(Parser, Debug, Copy, Clone)]
pub enum MouseButton {
    Left = 1,
    Right = 2,
    Center = 3,
    ScrollUp = 4,
    ScrollDown = 5,
}

#[derive(Parser, Debug)]
pub struct Move {
    #[clap(subcommand)]
    motion: Motion,
}

impl Move {
    fn move_x(&self, s: &mut State, xdo: &mut XDO, vp: (u32, u32)) {
        s.raise.0 += 1;

        let mut mv = f64::max(vp.0 as f64 / (1 << s.raise.0) as f64, 10.);

        if self.motion == Motion::Left {
            mv *= -1.;
        }

        xdo.relative_move(mv as i32, 0);
    }

    fn move_y(&self, s: &mut State, xdo: &mut XDO, vp: (u32, u32)) {
        s.raise.1 += 1;

        let mut mv = f64::max(vp.1 as f64 / (1 << s.raise.1) as f64, 10.);

        if self.motion == Motion::Top {
            mv *= -1.;
        }

        xdo.relative_move(0, mv as i32);
    }

    fn fire(&self, xdo: &mut XDO) -> Result<()> {
        let mut s = State::load().unwrap_or_default();

        let vp = xdo.viewport();

        match self.motion.axis() {
            Axis::X => self.move_x(&mut s, xdo, vp),
            Axis::Y => self.move_y(&mut s, xdo, vp),
        }

        s.save()?;

        Ok(())
    }
}

#[derive(Parser, Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum Motion {
    Top,
    Left,
    Right,
    Bottom,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum Axis {
    X,
    Y,
}

impl Motion {
    fn axis(&self) -> Axis {
        match self {
            Motion::Top | Motion::Bottom => Axis::Y,
            _ => Axis::X,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct State {
    raise: (usize, usize),
    hold: bool,
    mark: (i32, i32),
}

impl Default for State {
    fn default() -> Self {
        State {
            hold: false,
            raise: (1, 1),
            mark: (0, 0),
        }
    }
}

impl State {
    fn load() -> Result<State> {
        Ok(bincode::deserialize(&std::fs::read(
            "/dev/shm/mykeymouse",
        )?)?)
    }

    fn reset() -> Result<()> {
        let mut s = State::load().unwrap_or_default();

        s.raise = (1, 1);
        s.save()?;

        Ok(())
    }

    fn save(&self) -> Result<()> {
        std::fs::write("/dev/shm/mykeymouse", bincode::serialize(self)?)?;
        Ok(())
    }
}

fn main() -> Result<()> {
    let app = App::parse();
    let mut xdo = XDO::new().map_err(|_| anyhow!("Unable to use libxdo"))?;

    match app.command {
        Command::Screenshot => {
            let screen = x11_screenshot::Screen::open().ok_or(anyhow!("Unable to open screen"))?;

            let mut state = State::load().unwrap_or_default();

            let a = state.mark;

            let b = xdo.postition();

            let c = screen
                .capture_area(
                    (a.0 - b.0).abs() as u32,
                    (a.1 - b.1).abs() as u32,
                    std::cmp::min(a.0, b.0),
                    std::cmp::min(a.1, b.1),
                )
                .ok_or(anyhow!("Unable to capture screen"))?;

            c.save("/dev/shm/shoot.png")?;

            std::process::Command::new("xclip")
                .arg("-selection")
                .arg("clipboard")
                .arg("-t")
                .arg("image/png")
                .arg("-i")
                .arg("/dev/shm/shoot.png")
                .spawn()?
                .wait_with_output()?;

            state.mark = b;
            state.save()?;
        }

        Command::Centralize => {
            xdo.centralize();
            State::reset()?
        }
        Command::Click(ref c) => c.fire(xdo)?,
        Command::Move(ref m) => m.fire(&mut xdo)?,
        Command::Hold => {
            let mut s = State::load().unwrap_or_default();

            if s.hold {
                xdo.mouse_up();
                s.hold = false;
            } else {
                xdo.mouse_down();
                s.hold = true;
            }

            s.save()?;
        }
    }

    Ok(())
}
