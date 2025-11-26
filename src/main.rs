use leptos::prelude::*;

fn main() {
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    let (dark_mode, set_dark_mode) = signal(false);

    view! {
        <div class="page-wrapper" class:dark=dark_mode>
            <div class="page-border">
                <div class="container">
                    <Header dark_mode=dark_mode set_dark_mode=set_dark_mode/>
                    <main>
                        <Bio/>
                        <div class="sidebar">
                            <WorkHistory/>
                            <Education/>
                        </div>
                    </main>
                    <Footer/>
                </div>
            </div>
        </div>
    }
}

#[component]
fn Header(dark_mode: ReadSignal<bool>, set_dark_mode: WriteSignal<bool>) -> impl IntoView {
    let toggle_theme = move |_| set_dark_mode.set(!dark_mode.get());

    view! {
        <header>
            <nav>
                <a href="/" class="logo">
                    <svg xmlns="http://www.w3.org/2000/svg" width="22" height="22" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                        <path d="M3 9l9-7 9 7v11a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z"></path>
                        <polyline points="9 22 9 12 15 12 15 22"></polyline>
                    </svg>
                </a>
                <div class="nav-right">
                    <div class="nav-links">
                        <a href="https://github.com/DHolmanCoding" target="_blank" class="social-link" title="GitHub">
                            <svg xmlns="http://www.w3.org/2000/svg" width="22" height="22" viewBox="0 0 24 24" fill="currentColor">
                                <path d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z"/>
                            </svg>
                        </a>
                        <a href="https://www.linkedin.com/in/douglas-holman/" target="_blank" class="social-link" title="LinkedIn">
                            <svg xmlns="http://www.w3.org/2000/svg" width="22" height="22" viewBox="0 0 24 24" fill="currentColor">
                                <path d="M19 0h-14c-2.761 0-5 2.239-5 5v14c0 2.761 2.239 5 5 5h14c2.762 0 5-2.239 5-5v-14c0-2.761-2.238-5-5-5zm-11 19h-3v-11h3v11zm-1.5-12.268c-.966 0-1.75-.79-1.75-1.764s.784-1.764 1.75-1.764 1.75.79 1.75 1.764-.783 1.764-1.75 1.764zm13.5 12.268h-3v-5.604c0-3.368-4-3.113-4 0v5.604h-3v-11h3v1.765c1.396-2.586 7-2.777 7 2.476v6.759z"/>
                            </svg>
                        </a>
                    </div>
                    <button class="theme-toggle" on:click=toggle_theme>
                        {move || if dark_mode.get() { "☀️" } else { "🌙" }}
                    </button>
                </div>
            </nav>
        </header>
    }
}

#[component]
fn Bio() -> impl IntoView {
    view! {
        <section class="bio">
            <h1>"DOUGLAS "<span class="accent">"HOLMAN"</span></h1>
            <p class="intro">
                "I'm a hands-on leader specializing in building and scaling machine learning–powered products, working across scientific research, engineering, and product."
            </p>
        </section>
    }
}

#[component]
fn WorkHistory() -> impl IntoView {
    view! {
        <section class="work">
            <h2>"Work"</h2>
            <div class="work-list">
                <WorkItem
                    company="Flock Safety"
                    role="Principal Machine Learning Engineer"
                    period="2021 - 2025"
                />
                <WorkItem
                    company="Modzy (Booz Allen Hamilton)"
                    role="Senior Machine Learning Engineer"
                    period="2020 - 2021"
                />
                <WorkItem
                    company="Zymergen"
                    role="Data Scientist"
                    period="2019 - 2020"
                />
                <WorkItem
                    company="Oregon State University"
                    role="Deep Learning Researcher"
                    period="2018 - 2019"
                />
                <WorkItem
                    company="Oregon State University"
                    role="Genetics Researcher"
                    period="2016 - 2018"
                />
            </div>
        </section>
    }
}

#[component]
fn WorkItem(company: &'static str, role: &'static str, period: &'static str) -> impl IntoView {
    view! {
        <div class="work-item">
            <div class="work-info">
                <strong>{company}</strong>
                <span class="role">{role}</span>
            </div>
            <span class="period">{period}</span>
        </div>
    }
}

#[component]
fn Education() -> impl IntoView {
    view! {
        <section class="work education">
            <h2>"Education"</h2>
            <div class="work-list">

                <EduItem
                    school="University of Pennsylvania"
                    degree="Master of Computer and Information Technology"
                    focus="Machine Learning"
                />
                <EduItem
                    school="Oregon State University"
                    degree="Honors Bachelor of Science"
                    focus="Biochemistry and Biophysics"
                />
                <EduItem
                    school="Stanford University"
                    degree="Graduate Coursework"
                    focus="Human Physiology, Economics"
                />
            </div>
        </section>
    }
}

#[component]
fn EduItem(school: &'static str, degree: &'static str, focus: &'static str) -> impl IntoView {
    view! {
        <div class="work-item">
            <div class="work-info">
                <strong>{school}</strong>
                <span class="role">{degree}</span>
                <span class="focus">"Focus: "{focus}</span>
            </div>
        </div>
    }
}

#[component]
fn Footer() -> impl IntoView {
    view! {
        <footer>
            <p class="quote">"Thanks for stopping by my website."</p>
            <p class="built-with">"Built with Rust & Leptos"</p>
        </footer>
    }
}
