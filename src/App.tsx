import { FormEvent, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import {
  Activity,
  ArrowRight,
  CheckCircle2,
  Clapperboard,
  Compass,
  Film,
  Heart,
  Home,
  MonitorPlay,
  Play,
  Radio,
  Search,
  Settings2,
  ShieldCheck,
  Square,
  Tv,
  Wifi,
  XCircle,
} from "lucide-react";

const navigation = [
  { label: "Home", icon: Home },
  { label: "Live TV", icon: Tv },
  { label: "Movies", icon: Film },
  { label: "Series", icon: Clapperboard },
  { label: "Favorites", icon: Heart },
];

const shelves = [
  { title: "Live television", subtitle: "Open a stream from your provider", icon: Radio, tone: "amber" },
  { title: "Movies", subtitle: "A home for your movie library", icon: Film, tone: "violet" },
  { title: "Series", subtitle: "Pick up a show when you're ready", icon: Clapperboard, tone: "blue" },
];

function App() {
  const [activePage, setActivePage] = useState("Home");
  const [streamUrl, setStreamUrl] = useState("");
  const [busy, setBusy] = useState(false);
  const [status, setStatus] = useState("");
  const [statusKind, setStatusKind] = useState<"success" | "error" | "info">("info");
  const [searchText, setSearchText] = useState("");

  const filteredNavigation = navigation.filter((item) =>
    item.label.toLowerCase().includes(searchText.toLowerCase()),
  );

  async function handlePlay(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();
    const cleanUrl = streamUrl.trim();
    if (!cleanUrl) {
      setStatusKind("error");
      setStatus("Paste a direct stream URL first.");
      return;
    }

    setBusy(true);
    setStatus("");
    try {
      const response = await invoke<string>("play_stream", { streamUrl: cleanUrl });
      setStatusKind("success");
      setStatus(response);
    } catch (error) {
      setStatusKind("error");
      setStatus(String(error));
    } finally {
      setBusy(false);
    }
  }

  async function handleStop() {
    setBusy(true);
    try {
      const response = await invoke<string>("stop_stream");
      setStatusKind("info");
      setStatus(response);
    } catch (error) {
      setStatusKind("error");
      setStatus(String(error));
    } finally {
      setBusy(false);
    }
  }

  return (
    <div className="app-shell">
      <aside className="sidebar">
        <a className="brand" href="#" onClick={(event) => { event.preventDefault(); setActivePage("Home"); }}>
          <span className="brand-mark"><MonitorPlay size={23} strokeWidth={2.1} /></span>
          <span className="brand-copy"><strong>EliteStocks <em>TV</em></strong><small>DESKTOP PLAYER</small></span>
        </a>

        <div className="sidebar-label">WORKSPACE</div>
        <nav className="primary-nav" aria-label="Main navigation">
          {filteredNavigation.map(({ label, icon: Icon }) => (
            <button key={label} className={`nav-item ${activePage === label ? "active" : ""}`} onClick={() => setActivePage(label)}>
              <Icon size={18} strokeWidth={1.9} />
              <span>{label}</span>
              {label === "Live TV" && <span className="nav-live-dot" />}
            </button>
          ))}
        </nav>

        <div className="sidebar-bottom">
          <div className="engine-card">
            <span className="engine-icon"><Activity size={17} /></span>
            <div><strong>Playback engine</strong><span>MPV · Native process</span></div>
            <span className="engine-dot" />
          </div>
          <button className="nav-item secondary-nav" onClick={() => setActivePage("Settings")}><Settings2 size={18} /><span>Settings</span></button>
          <div className="profile-row">
            <div className="profile-avatar">ES</div>
            <div className="profile-copy"><strong>Desktop edition</strong><span>Local player</span></div>
            <span className="online-dot" />
          </div>
        </div>
      </aside>

      <main className="main-panel">
        <header className="topbar">
          <div className="breadcrumb"><span>ELITESTOCKS TV</span><span className="crumb-separator">/</span><strong>{activePage}</strong></div>
          <div className="topbar-actions">
            <label className="search-box">
              <Search size={16} />
              <input value={searchText} onChange={(event) => setSearchText(event.target.value)} placeholder="Search navigation" aria-label="Search navigation" />
              {searchText && <button onClick={() => setSearchText("")} aria-label="Clear search"><XCircle size={15} /></button>}
            </label>
            <div className="connection-pill"><span /> Desktop ready</div>
          </div>
        </header>

        <div className="page-content">
          <section className="hero">
            <div className="hero-art" aria-hidden="true">
              <div className="orb orb-one" />
              <div className="orb orb-two" />
              <div className="hero-screen"><div className="screen-glow" /><div className="screen-line line-one" /><div className="screen-line line-two" /><div className="screen-line line-three" /><div className="screen-glass" /></div>
              <div className="hero-orbit orbit-one" /><div className="hero-orbit orbit-two" />
            </div>
            <div className="hero-content">
              <div className="eyebrow"><span className="eyebrow-line" /> YOUR ENTERTAINMENT, ONE PLACE</div>
              <h1>Make room for<br /><span>what moves you.</span></h1>
              <p>A clean, focused home for your live streams and media. Your screen, your rules.</p>
              <button className="hero-button" onClick={() => document.getElementById("stream-url")?.focus()}>
                <Play size={16} fill="currentColor" /> Open a stream <ArrowRight size={16} />
              </button>
            </div>
            <div className="hero-index"><span>01</span><div /><span>03</span></div>
          </section>

          <section className="stream-panel">
            <div className="section-heading">
              <div><span className="section-kicker">QUICK START</span><h2>Play a stream</h2></div>
              <div className="engine-status"><span className="status-pulse" /> MPV media engine</div>
            </div>
            <form className="stream-form" onSubmit={handlePlay}>
              <div className="url-input-wrap"><Wifi size={18} /><input id="stream-url" type="url" value={streamUrl} onChange={(event) => setStreamUrl(event.target.value)} placeholder="Paste a direct stream URL (https://… or rtmp://…)" autoComplete="url" /></div>
              <button className="play-button" type="submit" disabled={busy}><Play size={16} fill="currentColor" /> {busy ? "Opening…" : "Play in MPV"}</button>
              <button className="stop-button" type="button" onClick={handleStop} disabled={busy} aria-label="Stop playback"><Square size={15} fill="currentColor" /></button>
            </form>
            {status && <div className={`status-message ${statusKind}`}>{statusKind === "success" ? <CheckCircle2 size={16} /> : <XCircle size={16} />}<span>{status}</span></div>}
            <p className="stream-hint"><ShieldCheck size={14} /> Use a direct stream URL from a source you're authorized to access. MPV must be installed and available on your Windows PATH.</p>
          </section>

          <section className="explore-section">
            <div className="section-heading explore-heading">
              <div><span className="section-kicker">YOUR LIBRARY</span><h2>{activePage === "Home" ? "A space to explore" : activePage}</h2></div>
              <button className="text-button" onClick={() => setActivePage("Live TV")}>Explore library <ArrowRight size={15} /></button>
            </div>
            <div className="shelf-grid">
              {shelves.filter((shelf) => !searchText || shelf.title.toLowerCase().includes(searchText.toLowerCase()) || activePage.toLowerCase().includes(searchText.toLowerCase())).map(({ title, subtitle, icon: Icon, tone }, index) => (
                <button className={`shelf-card ${tone}`} key={title} onClick={() => setActivePage(title === "Live television" ? "Live TV" : title)}>
                  <div className="shelf-card-top"><span className="shelf-icon"><Icon size={19} /></span><span className="shelf-number">0{index + 1}</span></div>
                  <div className="shelf-art" aria-hidden="true"><div className="shelf-ring" /><div className="shelf-ring ring-two" /><Icon size={68} strokeWidth={0.8} /></div>
                  <div className="shelf-card-copy"><strong>{title}</strong><span>{subtitle}</span></div>
                  <ArrowRight className="shelf-arrow" size={18} />
                </button>
              ))}
            </div>
            <div className="foundation-note"><span className="foundation-note-icon"><Compass size={17} /></span><div><strong>{activePage === "Home" ? "A fresh start, built for smooth navigation" : `${activePage} is part of the interface foundation`}</strong><p>Playlist import, EPG data, library browsing, and embedded MPV video rendering will be added in planned milestones. The current screen is the initial desktop shell.</p></div></div>
          </section>

          <footer className="app-footer"><span>ELITESTOCKS TV <b>·</b> DESKTOP</span><span>REACT <b>·</b> TAURI <b>·</b> WEBVIEW2 <b>·</b> MPV</span></footer>
        </div>
      </main>
    </div>
  );
}

export default App;
