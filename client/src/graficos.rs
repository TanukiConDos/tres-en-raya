use std::
    {
    sync::Arc,
    fs::File,
    path::Path,
    io::prelude::*
    };
use winit::dpi::PhysicalSize;
use crate::{
    juego::{
        Ficha,
        Tablero
    },
    Estado
};

/*
    Estructura que representa una imagen.
    Se compone de algunos de los datos de la cabezera del archivo
    y una matriz con los colores de los pixeles en formato RGBa.
*/
#[allow(unused)]
struct Image{
    file_type: [char;2],
    file_size: u32,
    width: u32,
    height: u32,
    bpp: u16,
    data: Vec<Vec<[u8;4]>>

}

/*
    Estructura que almacena las imagenes usadas como texturas
*/
pub struct Texturas
{
    menu_principal: Image,
    buscando: Image,
    casilla_x: Image,
    casilla_o: Image,
    turno: Image,
}

impl Texturas
{
    pub fn new(menu_principal: Vec<u8>,buscando: Vec<u8>,casilla_x: Vec<u8>,casilla_o: Vec<u8>,turno: Vec<u8>) -> Texturas
    {
        let menu_principal = Texturas::procesar(menu_principal);
        let buscando = Texturas::procesar(buscando);
        let casilla_x = Texturas::procesar(casilla_x);
        let casilla_o = Texturas::procesar(casilla_o);
        let turno = Texturas::procesar(turno);
        Texturas
        {
            menu_principal: menu_principal,
            buscando: buscando,
            casilla_x: casilla_x,
            casilla_o: casilla_o,
            turno: turno
        }
    }

    /*
        Procesa el buffer con los datos del archivo bmp
    */
    fn procesar(imagen: Vec<u8>) -> Image
    {
        //Saca los datos de la cabezera
        let file_type = [imagen[0] as char,imagen[1] as char];
        let file_size = ((imagen[5] as u32) << 24) + ((imagen[4] as u32) << 16) + ((imagen[3] as u32) << 8) + imagen[2] as u32;
        let offset = ((imagen[13] as u32) << 24) + ((imagen[12] as u32) << 16) + ((imagen[11] as u32) << 8) + imagen[10] as u32;
        let width = ((imagen[21] as u32) << 24) + ((imagen[20] as u32) << 16) + ((imagen[19] as u32) << 8) + imagen[18] as u32;
        let height = ((imagen[25] as u32) << 24) + ((imagen[24] as u32) << 16) + ((imagen[23] as u32) << 8) + imagen[22] as u32;
        let bpp = ((imagen[29] as u16) << 8) + imagen[28] as u16;

        let mut data: Vec<Vec<[u8;4]>> = Vec::with_capacity(height as usize);
        for _ in 0..data.capacity()
        {
            let mut row = Vec::with_capacity(width as usize);
            for _ in 0..row.capacity()
            {
                row.push([0;4]);
            }
            data.push(row);
        }
        let mut byte_padding = width * 3;
        {
            while byte_padding % 4 != 0
            {
                if byte_padding % 4 !=0
                {
                    byte_padding = byte_padding + 1;
                }
            }
            byte_padding = byte_padding - width * 3;
        }
        let image_size = width * height * 3;
        
        let byte_pixel = bpp as u32/8;
        let mut padding = 0;

        //Mete los datos de los pixeles de la imagen en una matriz
        for i in offset..image_size
        {
            if (i - offset) % (width * 3 + byte_padding) < (width * 3) 
            {
                let pixel = (i - offset - padding) / byte_pixel;
                let y = (pixel % width) as usize;
                let x = (height - 1 - (pixel / width)) as usize;
                
                if (i - offset - padding) % byte_pixel == 0 && y == 0 && x < height as usize - 1 
                {
                    padding = padding + byte_padding;
                    
                }
                
                
                if (i - offset - padding) % byte_pixel == 0
                {
                    
                    data[x][y] = [imagen[i as usize],imagen[i as usize + 1],imagen[i as usize + 2],0xff];
                }
            }
        }
        
        Image{
            file_type: file_type,
            file_size: file_size,
            width: width,
            height: height,
            bpp: bpp,
            data: data
        }
    }
}
pub fn cargar_texturas() -> Texturas
{
        let path_menu = Path::new("textures").join("menu_principal.bmp");
        let mut menu = File::open(path_menu).unwrap();
        let mut buf_menu = Vec::new();
        menu.read_to_end(&mut buf_menu).unwrap();

        let path_buscando = Path::new("textures").join("buscando_0.bmp");
        let mut buscando = File::open(path_buscando).unwrap();
        let mut buf_buscando = Vec::new();
        buscando.read_to_end(&mut buf_buscando).unwrap();

        let path_x = Path::new("textures").join("casilla_x.bmp");
        let mut casilla_x = File::open(path_x).unwrap();
        let mut buf_x = Vec::new();
        casilla_x.read_to_end(&mut buf_x).unwrap();

        let path_o = Path::new("textures").join("casilla_o.bmp");
        let mut casilla_o = File::open(path_o).unwrap();
        let mut buf_o = Vec::new();
        casilla_o.read_to_end(&mut buf_o).unwrap();

        let path_o = Path::new("textures").join("turno.bmp");
        let mut casilla_o = File::open(path_o).unwrap();
        let mut buf_t = Vec::new();
        casilla_o.read_to_end(&mut buf_t).unwrap();

        Texturas::new(buf_menu,buf_buscando,buf_x, buf_o,buf_t)

}

/*
    Dibuja en pantalla el tablero y si es el turno del jugador
*/
pub fn draw(frame: &mut [u8],size: PhysicalSize<u32>, tablero: Arc<std::sync::RwLock<Tablero>>,arc_turno: Arc<std::sync::RwLock<Ficha>>, texturas: &Texturas,estado: Arc<std::sync::RwLock<Estado>>)
{
    let estado = estado.read().unwrap();
    match *estado
    {
        Estado::MenuPrincipal => draw_menu(frame, size, texturas),
        Estado::Buscando => draw_buscando(frame,size,texturas),
        Estado::EnPartida => draw_tablero(frame, size, tablero, arc_turno, texturas),
    }
}

fn draw_menu(frame: &mut [u8],size: PhysicalSize<u32>,texturas: &Texturas)
{
    for (i, pixel) in frame.chunks_exact_mut(4).enumerate()
    {
        let i = i as u32;
        let y = i % size.width;
        let x = i / size.width;
        let rgba = texturas.menu_principal.data[x as usize][y as usize];
        pixel.copy_from_slice(&rgba);
    }
}

fn draw_buscando(frame: &mut [u8],size: PhysicalSize<u32>,texturas: &Texturas)
{
    for (i, pixel) in frame.chunks_exact_mut(4).enumerate()
    {
        let i = i as u32;
        let y = i % size.width;
        let x = i / size.width;
        let rgba = texturas.buscando.data[x as usize][y as usize];
        pixel.copy_from_slice(&rgba);
    }
}

fn draw_tablero(frame: &mut [u8],size: PhysicalSize<u32>, tablero: Arc<std::sync::RwLock<Tablero>>,arc_turno: Arc<std::sync::RwLock<Ficha>>, texturas: &Texturas)
{
    let celda_size = [((size.height - 30)/3),((size.width - 20)/3)];

    for (i, pixel) in frame.chunks_exact_mut(4).enumerate()
    {
        let i = i as u32;
        let y = i % size.width;
        let x = i / size.width;

        let inside = x > 20 && x < size.height - 10
                    && y > 10 && y < size.width - 10;
        
        let tablero = tablero.read().unwrap();
        let rgba = if inside
        {
            let celda_x = (x as usize - 21) / celda_size[0] as usize;
            let celda_y = (y as usize - 11) / celda_size[1] as usize;
            let y = (y - 11) % celda_size[1];
            let x = (x - 21) % celda_size[0];
            
            let inside = y < celda_size[1] -2
                        && x < celda_size[0] -2;
            if inside
            {
                match tablero.get_celda(celda_x,celda_y)
                {
                    Ficha::X =>
                    {
                        texturas.casilla_x.data[x as usize][y as usize]
                    },
                    Ficha::O =>
                    {
                        texturas.casilla_o.data[x as usize][y as usize]
                    }
                    Ficha::No => [0xff, 0xff, 0xff, 0xff]
                }
            }
            else
            {
                [0, 0, 0, 0xff]
            }
        }
        else
        {
            let turno = *arc_turno.read().unwrap();
            if x > 0 && x < 17 && y > 0 && y < 63 && tablero.get_turno() == turno
            {
                texturas.turno.data[x as usize - 1][y as usize - 1]
            }
            else
            {
                [0, 0, 0, 0xff]
            }
            
        };
        pixel.copy_from_slice(&rgba);
    }
}