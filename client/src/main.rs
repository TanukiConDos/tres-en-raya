#![windows_subsystem = "windows"]

use winit::
{
    event::
    {
        Event,
        WindowEvent,
        DeviceEvent,
        VirtualKeyCode,
        ElementState,
        MouseButton
    },
    event_loop::
    {
        ControlFlow,
        EventLoop
    },
    window::WindowBuilder,

};
use pixels::
{
    Pixels,
    SurfaceTexture
};

use client::
{
    juego::
    {
        Tablero,
        Ficha,
    },
    graficos,
    Estado
};

use std::
{
    net::{TcpStream, Shutdown},
    mem::transmute,
    thread,
    sync::
    {
        Arc,
        RwLock,
        Mutex,
    },
    io::{Write, Read},
};

#[allow(deprecated)]
fn main() {

    let estado = Arc::new(RwLock::new(Estado::MenuPrincipal));

    //Configuracion de la ventana
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
                .with_title("Tres en raya")
                .with_resizable(false)
                .with_inner_size(winit::dpi::Size::Physical(winit::dpi::PhysicalSize::new(320,330)))
                .build(&event_loop)
                .unwrap();
    let inner_size = window.inner_size();

    let tablero: Tablero = Tablero::new();
    let turno = Arc::new(RwLock::new(Ficha::No));
    let tablero = Arc::new(RwLock::new(tablero));
    let enviar = Arc::new(Mutex::new(false));

    let enviar_thread = enviar.clone();
    let tablero_thread = tablero.clone();
    let estado_thread = estado.clone();
    

    let turno_thread = turno.clone();
    
    //Hilo para la conexion con el servidor
    thread::spawn(move ||
    {
        let arc_tablero = tablero_thread;
        let arc_estado = estado_thread;
        let arc_enviar = enviar_thread;
        let arc_turno = turno_thread;
        loop
        {
            let arc_estado = arc_estado.clone();
            while *arc_estado.read().unwrap() != Estado::Buscando
            {

            }
            
            //Conexion con el servidor
            let mut stream = TcpStream::connect("192.168.1.11:7878").unwrap();
            stream.write("conection request".as_bytes()).unwrap();
            
            {//recibe la ficha del jugador
                let mut buf = [0;std::mem::size_of::<Ficha>()];
                stream.read(&mut buf).unwrap();
                *arc_turno.write().unwrap() = unsafe {transmute(buf)};
            }
            *arc_estado.write().unwrap() = Estado::EnPartida;
            
            let arc_tablero = arc_tablero.clone();
            let mut buf = [0;std::mem::size_of::<Tablero>()];

            let w_stream = stream.try_clone().unwrap();
            let tablero_thread = arc_tablero.clone();
            let arc_enviar = arc_enviar.clone();
            let arc_estado = arc_estado.clone();
            thread::spawn(move ||
            {
                let mut stream = w_stream;
                let arc_tablero = tablero_thread;
                loop{
                    if *arc_estado.read().unwrap() == Estado::MenuPrincipal
                    {
                        stream.write(&[0]).unwrap();
                        stream.shutdown(Shutdown::Both).unwrap();
                        return;
                    }
                    let mut enviar = arc_enviar.lock().unwrap();
                    if *enviar == true
                    {
                        *enviar = false;
                        let tablero = *arc_tablero.read().unwrap();
                        let paquete = unsafe {std::slice::from_raw_parts((&tablero as *const Tablero) as *const u8, std::mem::size_of::<Tablero>())};
                        stream.write(paquete).unwrap();
                        
                    }
                }
                
            });

            loop
            {
                
                let n = stream.read(&mut buf).unwrap_or(0);
                let mut tablero = arc_tablero.write().unwrap();
                if n == std::mem::size_of::<Tablero>()
                {
                    if !tablero.is_finished(){
                        tablero.new_tablero(unsafe {transmute(buf)});
                    }
                    else
                    {
                        break;
                    }
                }
                if n==0 {
                    break;
                }
            }
        }
    });

    let tablero_thread = tablero.clone();
    let surface_texture = SurfaceTexture::new(inner_size.width, inner_size.height, &window);
    let pixels = Pixels::new(inner_size.width, inner_size.height, surface_texture).unwrap();

    let arc_turno = turno.clone();
    let arc_estado = estado.clone();

    //Hilo encargado de renderizar el tablero y mostrarlo en pantalla
    thread::spawn(move ||
    {
        let mut pixels = pixels;
        let arc_tablero = tablero_thread;
        let texturas = graficos::cargar_texturas();

        loop
        {
            graficos::draw(pixels.get_frame(),inner_size,arc_tablero.clone(),arc_turno.clone(),&texturas,arc_estado.clone());
            pixels.render().unwrap();
        }
    });

    let mut focus = true;
    let mut cursor: [u16;2] = [0,0];

    //Loop para tratar los eventos de la ventana y el ratón
    event_loop.run(move |event, _, control_flow|
        {
        match event
        {
            Event::WindowEvent
            {
                event: WindowEvent::CloseRequested,
                window_id,
            }
            if window_id == window.id() => *control_flow = ControlFlow::Exit,
            Event::DeviceEvent
            {
                device_id: _,
                event: DeviceEvent::Key(key),
            }
            if key.virtual_keycode == Some(VirtualKeyCode::Escape) && focus => *control_flow = ControlFlow::Exit,
            Event::WindowEvent
            {
                event: WindowEvent::Focused(focused),
                window_id,
            }
            if window_id == window.id() => focus = focused,
            Event::WindowEvent
            {
                event: WindowEvent::CursorMoved
                {
                    device_id: _,
                    position,
                    modifiers: _,
                },
                window_id,
            }
            if window_id == window.id() => cursor = [position.y as u16,position.x as u16],
            Event::WindowEvent
            {
                event: WindowEvent::MouseInput
                {
                    device_id: _,
                    state: ElementState::Released,
                    button: MouseButton::Left,
                    modifiers: _,
                },
                window_id,
            } 
            if window_id == window.id() && focus =>
            {
                let mut estado = estado.write().unwrap();
                match *estado
                {
                    Estado::MenuPrincipal => 
                    {
                        if cursor[0] >= 196 && cursor[0] <= 267 && cursor[1] >= 81 && cursor[1] <= 251
                        {
                            *estado = Estado::Buscando
                        }
                    }
                    Estado::Buscando => {}
                    Estado::EnPartida =>
                    {   
                        let mut tablero = tablero.write().unwrap();
                        if tablero.is_finished()
                        {
                            *estado = Estado::MenuPrincipal;
                            *tablero = Tablero::new();
                        }else
                        {
                            let turno = *turno.read().unwrap();
                            tablero.put_piece(cursor,inner_size,&turno);
                            if tablero.get_turno() != turno
                            {
                                *enviar.lock().unwrap() = true;
                            }
                        }
                    }
                }
            }
            _ => (),
        }
    });
}