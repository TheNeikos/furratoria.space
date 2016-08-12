
use std::borrow::Cow;
use std::fmt;

use maud::PreEscaped;
use iron::Url;
use iron::Request;
use mount::OriginalUrl;
use iron_login::User as UserTrait;

use views::components::Navbar;
use models::user::User;

pub struct LayoutData {
    pub url: Url,
    pub user: Option<User>,
}

impl LayoutData {
    pub fn from_request(req: &mut Request) -> LayoutData {
        LayoutData {
            url: req.extensions.get::<OriginalUrl>().unwrap().clone(),
            user: User::get_login(req).get_user(),
        }
    }
}


pub fn application(mut data: &mut fmt::Write,
                   title: Cow<str>,
                   partial: Cow<str>,
                   layout_data: &LayoutData,
                   ) -> Result<(), fmt::Error>
{
    html!(data, {
        html {
            head {
                title ^title
                link rel="stylesheet" href="https://maxcdn.bootstrapcdn.com/bootstrap/4.0.0-alpha.3/css/bootstrap.min.css" integrity="sha384-MIwDKRSSImVFAZCVLtU0LMDdON6KVCrZHyVQQj6e8wIEJkW4tvwqXrbMIya1vriY" crossorigin="anonymous" /
                script src="https://maxcdn.bootstrapcdn.com/bootstrap/4.0.0-alpha.3/js/bootstrap.min.js" integrity="sha384-ux8v3A6CPtOTqOzMKiuo3d/DomGaaClxFYdCu2HPMBEkf6x2xiDyJ7gkXU0MWwaD" crossorigin="anonymous" ""
            }

            body {
                div.container-fluid {
                    ^PreEscaped(Navbar::new(&layout_data))

                    ^PreEscaped(partial)

                    hr /
                    footer {
                        p ^PreEscaped("ArtMoe 2016 &copy; Neikos")
                    }
                }
            }
        }
    })
}
