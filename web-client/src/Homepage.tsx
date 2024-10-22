// HomePage.tsx
import React from "react";

const HomePage: React.FC = () => {
    return (
        <>
            <section className="py-2 md:py-4 font-sans text-center justify-center items-center max-w-5xl mt-20">
                <div className="items-center flex text-center">
                    <div className="flex justify-center border p-1 rounded-lg text-center">
                        <a
                            href="https://github.com/db-keli/shinobi/issues"
                            className="font-geist"
                        >
                            contribute to project
                        </a>
                    </div>
                </div>
                <div className="items-center mt-2 flex text-center">
                    <div className="flex justify-center text-center">
                        <p className="font-geist text-5xl mr-2">Shinobi</p>
                    </div>
                    <div>
                        <p className="text-5xl">忍</p>
                    </div>
                </div>

                <div className="flex p-3 justify-center items-center">
                    <p className="font-geist mt-4 flex w-3/4 text-gray-400 font-extrabold text-3xl">
                        Shinobi is a secure client-server tool designed to help
                        project owners manage builds without exposing sensitive
                        information.
                    </p>
                </div>

                <div className="flex p-3 justify-center items-center hover:mb-2 hover:-mt-2 transition-all">
                    <p className="font-geist mt-4 flex border p-6 rounded-lg w-3/4 font-light opacity-70 text-2xl">
                        With Shinobi, you do not have to share your keys with
                        freelancers or third-party developers to build a
                        project. It encrypts sensitive keys on the server side,
                        generates a secure token, and encodes it into a QR code.
                        Clients can scan this QR code to set up projects without
                        directly handling sensitive data. This approach ensures
                        safe and streamlined management of project credentials
                        in collaborative development environments.
                    </p>
                </div>

                <div className="flex-col justify-center items-center">
                    <div>
                        <p className="font-geist mt-4 text-4xl">
                            Why did I make Shinobi?
                        </p>

                        <p className="font-geist mt-2 text-2xl mx-2 opacity-55">
                            With Shinobi, you do not have to share your keys
                            with freelancers or third-party developers to build
                            a project. It encrypts sensitive keys on the server
                            side, generates a secure token, and encodes it into
                            a QR code. Clients can scan this QR code to set up
                            projects without directly handling sensitive data.
                            This approach ensures safe and streamlined
                            management of project credentials in collaborative
                            development environments.
                        </p>
                    </div>
                </div>
            </section>
        </>
    );
};

export default HomePage;
