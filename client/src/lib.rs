pub mod graficos;
pub mod juego;

#[derive(PartialEq)]
pub enum Estado{
    MenuPrincipal,
    Buscando,
    EnPartida,
}