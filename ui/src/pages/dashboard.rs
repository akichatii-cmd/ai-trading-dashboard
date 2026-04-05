use leptos::*;
use crate::components::*;

#[component]
pub fn Dashboard() -> impl IntoView {
    view! {
        <div class="h-screen flex flex-col bg-void">
            <Header/>
            
            <div class="flex-1 grid grid-cols-[300px_1fr_300px] gap-4 p-4 min-h-0">
                <LeftPanel/>
                <CenterPanel/>
                <RightPanel/>
            </div>
            
            <Footer/>
        </div>
    }
}
