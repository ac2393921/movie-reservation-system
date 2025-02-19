pub trait ReserveTicketUseCase {
    fn exec(&self) -> Result<Ticket, Error>;
}

pub struct ReserveTicketInteractor<RR> {
    ReservationReposiotry: RR,
}

#[async_trait]
impl<RR> ReserveTicketUseCase for ReserveTicketInteractor<RR> {
    pub fn new(reservation_repository: RR) -> Self {
        ReserveTicketInteractor {
            ReservationReposiotry: reservation_repository,
        }
    }

    pub async fn exec(&self) -> Result<Ticket, Error> {
        let resavation = Resavation::new();
        self.ReservationReposiotry.save(resavation).await
    }
}
