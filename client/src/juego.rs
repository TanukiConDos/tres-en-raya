use winit::dpi::PhysicalSize;
use std::fmt;

/*
    Enumerado con el tipo de ficha
*/
#[derive(PartialEq,Copy,Clone)]
pub enum Ficha {
    X,
    O,
    No
}

impl fmt::Display for Ficha{
    fn fmt(&self,f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        match self{
            Ficha::X => write!(f,"{}","X"),
            Ficha::O => write!(f,"{}","O"),
            Ficha::No => write!(f,"{}"," "),
        }
    }
}

/*
    Estructura que representa el tablero de juego
*/
#[derive(Copy, Clone)]
pub struct Tablero
{
    celdas: [[Ficha;3];3],
    turno: Ficha
}

impl Tablero
{
    pub fn new() -> Tablero
    {
        Tablero
        {
            celdas: [[Ficha::No,Ficha::No,Ficha::No],[Ficha::No,Ficha::No,Ficha::No],[Ficha::No,Ficha::No,Ficha::No]],
            turno: Ficha::X,
        }
    }

    /*
        Sustituye los datos del tablero por los del tableroç
        pasado por parametro
    */
    pub fn new_tablero(&mut self,t: Tablero)
    {
        self.celdas = t.celdas;
        self.turno = t.turno;
    }

    pub fn get_turno(&self) -> Ficha
    {
        self.turno
    }

    pub fn get_celda(&self,x: usize,y: usize) -> Ficha
    {
        self.celdas[x][y]
    }
    
    pub fn put_piece(&mut self,cursor: [u16;2],size: PhysicalSize<u32> , player: &Ficha)
    {
        let celda_size = [((size.height - 20)/3) as u16,((size.width - 20)/3) as u16];

        //Comprueba que el clic del raton esta dentro del tablero
        let inside = cursor[0] > 10 && cursor[0] < size.height as u16 - 10
                    && cursor[1] > 10 && cursor[1] < size.width as u16 - 10;
        if !inside
        {
            return;
        }

        //Comprueba que el clic del raton esta dentro de una celda
        //y se calcula cual
        let y = (cursor[1] - 11) % celda_size[1];
        let x = (cursor[0] - 11) % celda_size[0];

        let inside = y < celda_size[1] - 2
                    && x < celda_size[0] - 2;
        if !inside
        {
            return;
        }

        let x = (cursor[0] - 11) / celda_size[0];
        let y = (cursor[1] - 11) / celda_size[1];

        //Si es el turno del jugador y la celda esta vacia se coloca la pieza
        //y se manda el tablero al servidor
        if let Ficha::No = self.celdas[x as usize][y as usize]
        {
            if player == &self.turno
            {
                match self.turno
                {
                    Ficha::X =>
                    {
                        self.celdas[x as usize][y as usize] = Ficha::X;
                        self.turno = Ficha::O;
                    },
                    
                    Ficha::O =>
                    {
                        self.celdas[x as usize][y as usize] = Ficha::O;  
                        self.turno = Ficha::X;
                    },

                    Ficha::No => panic!(),
                }
            }
        }
    }

    pub fn is_finished(&self) -> bool{
        let mut count_o = [0;8];
        let mut count_x = [0;8];
        let mut count_no = 0;
        for i in 0..3
        {
            match self.celdas[i][1]
            {
                Ficha::O => count_o[0] = count_o[0] + 1,
                Ficha::X => count_x[0] = count_x[0] + 1,
                Ficha::No => {count_no = count_no + 1}
            }
            match self.celdas[1][i]
            {
                Ficha::O => count_o[1] = count_o[1] + 1,
                Ficha::X => count_x[1] = count_x[1] + 1,
                Ficha::No => {count_no = count_no + 1}
            }
            match self.celdas[i][i]
            {
                Ficha::O => count_o[2] = count_o[2] + 1,
                Ficha::X => count_x[2] = count_x[2] + 1,
                Ficha::No => {count_no = count_no + 1}
            }
            match self.celdas[2-i][i]
            {
                Ficha::O => count_o[3] = count_o[3] + 1,
                Ficha::X => count_x[3] = count_x[3] + 1,
                Ficha::No => {count_no = count_no + 1}
            }
            match self.celdas[i][0]
            {
                Ficha::O => count_o[4] = count_o[4] + 1,
                Ficha::X => count_x[4] = count_x[4] + 1,
                Ficha::No => {count_no = count_no + 1}
            }
            match self.celdas[i][2]
            {
                Ficha::O => count_o[5] = count_o[5] + 1,
                Ficha::X => count_x[5] = count_x[5] + 1,
                Ficha::No => {count_no = count_no + 1}
            }
            match self.celdas[0][i]
            {
                Ficha::O => count_o[6] = count_o[6] + 1,
                Ficha::X => count_x[6] = count_x[6] + 1,
                Ficha::No => {count_no = count_no + 1}
            }
            match self.celdas[2][i]
            {
                Ficha::O => count_o[7] = count_o[7] + 1,
                Ficha::X => count_x[7] = count_x[7] + 1,
                Ficha::No => {count_no = count_no + 1}
            }
            
        }
        
        for i in 0..8
        {
            if count_o[i] == 3 || count_x[i] == 3
            {
                return true
            }
        }
    
        if count_no == 0
        {
            return true
        }
        false
    }
}

impl fmt::Display for Tablero
{
    fn fmt(&self,f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        write!(f,"| {} | {} | {} |\n| {} | {} | {} |\n| {} | {} | {} |\n",self.celdas[0][0],self.celdas[0][1],self.celdas[0][2],self.celdas[1][0],self.celdas[1][1],self.celdas[1][2],self.celdas[2][0],self.celdas[2][1],self.celdas[2][2])
    }
}