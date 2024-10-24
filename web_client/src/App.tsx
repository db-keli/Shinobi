// App.tsx
import { BrowserRouter as Router, Route, Routes, Link } from "react-router-dom";
import Changelog from "./Changelog";
import HomePage from "./Homepage";

function App() {
    return (
        <Router>
            <nav className="py-2 md:py-4 items-end mx-auto justify-end mt-20">
                <div className="flex gap-4 items-end mx-auto max-w-5xl justify-end">
                    <Link to="/" className="font-geist text-base">
                        home
                    </Link>
                    <Link to="/changelog" className="font-geist text-base">
                        changelog
                    </Link>
                </div>
            </nav>
            <section className="flex flex-col items-center justify-center py-2 md:py-4 max-w-5xl mx-auto text-left">
                <Routes>
                    <Route path="/" element={<HomePage />} />

                    <Route path="/changelog" element={<Changelog />} />
                </Routes>
            </section>
            <footer className="mt-32">
                <section className="py-2 md:py-4 max-w-5xl mx-auto">
                    <div className="max-w-5xl bottom-0 absolute mb-8 flex flex-col md:flex-row gap-4 justify-between items-center">
                        <div className="flex gap-4">
                            <a
                                href="https://github.com/db-keli"
                                target="_blank"
                                rel="noopener noreferrer"
                            >
                                <svg
                                    xmlns="http://www.w3.org/2000/svg"
                                    width="24"
                                    height="24"
                                    viewBox="0 0 24 24"
                                    fill="none"
                                >
                                    <path
                                        d="M10 14.9993C9.34732 15.6987 8.98919 16.6227 9 17.5793V20.9993M14 14.9993C14.6527 15.6987 15.0108 16.6227 15 17.5793V20.9993M9 19.0493C8.10549 19.4055 7.13532 19.5294 6.18 19.4093C4.66 18.8893 5.06 17.5093 4.28 16.9393C3.90518 16.6713 3.46037 16.5184 3 16.4993M19 9.74927C19 12.7493 17.05 14.9993 12 14.9993C6.95 14.9993 5 12.7493 5 9.74927C4.9753 8.70844 5.20893 7.67772 5.68 6.74927C5.34 5.27927 5.47 3.46927 6.2 3.10927C6.93 2.74927 8.47 3.40927 9.74 4.25927C10.486 4.12615 11.2422 4.05922 12 4.05927C12.7572 4.05262 13.5134 4.11285 14.26 4.23927C15.53 3.38927 17.14 2.75927 17.8 3.08927C18.46 3.41927 18.66 5.25927 18.32 6.72927C18.7943 7.66371 19.028 8.70171 19 9.74927Z"
                                        stroke="currentcolor"
                                        stroke-width="2"
                                        stroke-linecap="round"
                                        stroke-linejoin="round"
                                    ></path>
                                </svg>
                            </a>
                        </div>
                        <div>
                            <a
                                href="https://www.buymeacoffee.com/dompehbright"
                                target="_blank"
                            >
                                <img
                                    src="https://cdn.buymeacoffee.com/buttons/v2/default-yellow.png"
                                    alt="Buy Me A Coffee"
                                    height={"30px"}
                                    width={"217px"}
                                />
                            </a>
                        </div>
                    </div>
                </section>
            </footer>
        </Router>
    );
}

export default App;
