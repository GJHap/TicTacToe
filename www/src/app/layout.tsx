import type { Metadata } from "next";
import "./globals.css";

export const metadata: Metadata = {
    title: "TicTacToe",
};

export default function RootLayout({
    children,
}: Readonly<{
    children: React.ReactNode;
}>) {
    return (
        <html lang="en" className="h-full box-border">
            <body className="h-full m-0">{children}</body>
        </html>
    );
}
