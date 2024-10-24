import React, { useEffect, useState } from "react";
import ReactMarkdown from "react-markdown";

const Changelog: React.FC = () => {
    const [changelog, setChangelog] = useState<string>("");
    const [loading, setLoading] = useState<boolean>(true);

    useEffect(() => {
        const fetchChangelog = async () => {
            try {
                const response = await fetch("/CHANGELOG.md");

                if (response.ok) {
                    const markdown = await response.text();
                    setChangelog(markdown);
                } else {
                    console.error("Failed to load changelog:", response.status);
                }
            } catch (error) {
                console.error("Error loading changelog:", error);
            } finally {
                setLoading(false);
            }
        };

        fetchChangelog();
    }, []);

    return (
        <div className="p-4">
            <h1 className="text-3xl max-sm:text-xl font-geist text-red-800 mb-4">
                Changelog
            </h1>
            {loading ? (
                <p className="font-geist">Loading changelog...</p>
            ) : (
                <ReactMarkdown
                    className={"prose font-geist max-sm:text-sm text-base"}
                >
                    {changelog}
                </ReactMarkdown>
            )}
        </div>
    );
};

export default Changelog;
