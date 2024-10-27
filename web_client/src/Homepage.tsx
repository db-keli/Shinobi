// HomePage.tsx
import React from "react";

const HomePage: React.FC = () => {
    return (
        <>
            <section className="py-2 md:py-4 text-center justify-center items-center max-w-5xl">
                <div className="items-center mb-2 flex text-red-800 text-center">
                    <div className="flex justify-center text-center">
                        <p className="font-geist text-2xl max-sm:text-lg mr-2">
                            shinobi
                        </p>
                    </div>
                    <div>
                        <p className="text-2xl max-sm:text-lg">忍</p>
                    </div>
                </div>
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

                <div className="flex p-3 justify-center items-center">
                    <p className="font-geist mt-4 text-left max-sm:text-xs flex w-4/4 text-gray-400 font-medium text-lg">
                        shinobi is a secure client-server tool designed to help
                        project owners manage builds without exposing sensitive
                        information.
                    </p>
                </div>

                <div className="flex p-3 justify-center items-center">
                    <p className="font-geist mt-4 flex text-left w-4/4 max-sm:text-xs font-light opacity-80 text-lg">
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
                        <p className="font-geist mx-2 text-red-800 max-sm:text-lg text-left mt-4 text-2xl">
                            why did i make shinobi?
                        </p>

                        <p className="font-geist mt-2 pb-1 text-lg mx-2 text-left max-sm:text-xs opacity-55">
                            I've come across a numerous situations where
                            freelancers and developer end up blackmailing
                            project owners by holding their project keys
                            hostage. And this is a way I choose to solve that
                            problem.
                        </p>
                        <p className="text-left">
                            <a
                                className="border px-1 py-1 rounded-lg mx-2 text-left font-geist text-xs font-semibold"
                                href="https://security.stackexchange.com/questions/213511/how-to-securely-share-sensitive-info-such-as-api-keys-with-freelancers"
                            >
                                see an example
                            </a>
                        </p>
                    </div>
                </div>
            </section>
        </>
    );
};

export default HomePage;
