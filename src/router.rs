use crate::router::ViewState::Title;
use crate::view::title::TitleState;

enum ViewState {
    Title { state: TitleState }
}

enum Ticket {
    Play40Line,
}

impl Ticket {
    fn go(&self) -> ViewState {
        match &self {
            Ticket::Play40Line => Title { state: TitleState {} } // TODO: implement
        }
    }
}

enum Next {
    Continue { state: ViewState },
    Transit { ticket: Ticket },
    Exit,
}