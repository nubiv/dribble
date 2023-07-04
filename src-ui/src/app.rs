use leptos::{component, create_signal, provide_context, view, IntoView, Scope, SignalGet};

use crate::landing_page::LandingPage;
use crate::meeting::Meeting;

#[derive(Clone)]
pub struct InMeetingContext(pub leptos::WriteSignal<bool>);

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    let (in_meeting, set_in_meeting) = create_signal(cx, false);
    provide_context(cx, InMeetingContext(set_in_meeting));

    view! { cx,
            <main class="mx-[30%] mt-20">
            {
                move || if in_meeting.get() {
                    view! { cx,
                        <Meeting/>
                    }
                } else {
                    view! { cx,
                        <LandingPage/>
                    }
                }
            }
            </main>
    }
}
