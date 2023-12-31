use game::Game;
#[cfg(any(target_os = "windows", target_os = "linux"))]
use vulkan_renderer::LazyVulkan;

#[cfg(target_os = "macos")]
use metal_renderer::MetalRenderer;

use common::{
    log,
    winit::{
        self,
        event::{Event, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        platform::run_return::EventLoopExtRunReturn,
    },
    Renderer,
};

const INITIAL_SCREEN_WIDTH: u32 = 1000;
const INITIAL_SCREEN_HEIGHT: u32 = 1000;

fn load_game_from_disk() -> Result<Game, common::anyhow::Error> {
    let game_json = std::fs::read("game.json")?;
    log::info!("game.json found! Loading from disk..");
    let value: common::serde_json::Value = common::serde_json::from_slice(&game_json).unwrap();
    let game = game::Game::from_json(&value)?;
    log::info!("Loading complete!");
    Ok(game)
}

pub fn init<R: Renderer>() -> (
    R,
    EventLoop<()>,
    gui::GUI,
    game::Game,
    yakui_winit::YakuiWinit,
) {
    env_logger::init();
    log::debug!("Debug logging enabled");
    let event_loop = winit::event_loop::EventLoop::new();
    let size = winit::dpi::LogicalSize::new(INITIAL_SCREEN_WIDTH, INITIAL_SCREEN_HEIGHT);

    let window = winit::window::WindowBuilder::new()
        .with_inner_size(size)
        .with_title("Clipper".to_string())
        .build(&event_loop)
        .unwrap();

    let mut game = load_game_from_disk().unwrap_or(game::init());

    game.resized(window.inner_size());
    let yak_winit = yakui_winit::YakuiWinit::new(&window);

    let renderer = R::init(window);
    let gui = gui::GUI::new(INITIAL_SCREEN_WIDTH, INITIAL_SCREEN_HEIGHT);

    (renderer, event_loop, gui, game, yak_winit)
}

#[cfg(target_os = "macos")]
type RendererImpl = MetalRenderer;

#[cfg(any(target_os = "windows", target_os = "linux"))]
type RendererImpl = LazyVulkan;

fn main() {
    println!("Starting clipper!");
    let (mut renderer, mut event_loop, mut gui, mut game, mut yak_winit) = init::<RendererImpl>();
    let mut asset_loader = asset_loader::AssetLoader::new();

    // Off we go!
    let mut winit_initializing = true;
    event_loop.run_return(|event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        let handled_by_yak = yak_winit.handle_event(&mut gui.yak, &event);
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                *control_flow = ControlFlow::Exit;
            }
            Event::NewEvents(cause) => {
                if cause == winit::event::StartCause::Init {
                    winit_initializing = true;
                } else {
                    winit_initializing = false;
                }
            }

            Event::MainEventsCleared => {
                window_tick(&mut game, &mut renderer, &mut gui, &mut asset_loader);
            }
            Event::WindowEvent {
                event: WindowEvent::Resized(size),
                ..
            } => {
                if winit_initializing {
                    return;
                } else {
                    game.resized(size);
                    gui.resized(size.width, size.height);
                    renderer.resized(size);
                }
            }
            Event::WindowEvent { event, .. } => {
                if !handled_by_yak {
                    game::handle_winit_event(&mut game, event)
                }
            }
            _ => (),
        }
    });

    log::info!("Game is shutting down..");
    let game_json = common::serde_json::to_value(&game).unwrap();
    std::fs::write("game.json", game_json.to_string()).unwrap();
    renderer.cleanup();
}

fn window_tick<R: Renderer>(
    game: &mut game::Game,
    renderer: &mut R,
    gui: &mut gui::GUI,
    asset_loader: &mut asset_loader::AssetLoader,
) {
    game.time.start_frame();
    let needs_restart = game::tick(game, &mut gui.state);
    asset_loader.load_assets(&mut game.world);
    game.input.camera_zoom = 0.;
    gui::draw_gui(gui);

    if needs_restart {
        println!("Game needs restart!");
        game.resized(renderer.window().inner_size());
    }
    renderer.update_assets(&mut game.world);
    renderer.render(
        &game.world,
        &game.debug_lines,
        game.camera,
        &mut gui.yak,
        game.clock.time_of_day(),
    );
}
