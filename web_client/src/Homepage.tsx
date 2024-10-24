// HomePage.tsx
import React from "react";

const HomePage: React.FC = () => {
    return (
        <>
            <section className="py-2 md:py-4 text-center justify-center items-center max-w-5xl">
                <div className="items-center flex text-center">
                    <div className="flex justify-center border rounded-md text-center">
                        <a
                            href="https://github.com/db-keli/shinobi/issues"
                            className="font-geist text-xs font-semibold p-2"
                        >
                            contribute to project
                        </a>
                    </div>
                </div>
                <div className="items-center mt-5 flex text-center">
                    <div className="flex justify-center text-center">
                        <p className="font-geist text-5xl mr-2">shinobi</p>
                    </div>
                    <div>
                        <p className="text-5xl">忍</p>
                    </div>
                </div>

                <div className="flex p-3 justify-center items-center">
                    <p className="font-geist mt-4 flex w-3/4 text-gray-400 font-semibold text-2xl">
                        shinobi is a secure client-server tool designed to help
                        project owners manage builds without exposing sensitive
                        information.
                    </p>
                </div>

                <div className="flex p-3 justify-center items-center">
                    <p className="font-geist mt-4 flex border p-6 rounded-lg w-3/4 font-light opacity-80 text-xl">
                        ever thought of sharing your secret project keys and
                        worrying someone might mess things up? with shinobi, you
                        don’t have to! it locks your keys in a digital vault and
                        wraps that vault in a qr code. all you do is hand over
                        the qr code, and they can set up the project without
                        ever seeing the keys or struggling to build it. it’s
                        like giving someone a magic button to build the
                        project—no keys, no hassle, just done!
                    </p>
                </div>

                <div className="flex-col justify-center items-center">
                    <div>
                        <p className="font-geist mt-4 text-3xl">
                            why did i make shinobi?
                        </p>

                        <p className="font-geist mt-2 text-xl mx-2 opacity-55">
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
