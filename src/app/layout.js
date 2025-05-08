import { Geist, Geist_Mono } from "next/font/google";
import "../globals.css";
import Navbar from "@/components/navbar/navbar";
import Ender from "@/components/footer/ender";

const geistSans = Geist({
	variable: "--font-geist-sans",
	subsets: ["latin"],
});

const geistMono = Geist_Mono({
	variable: "--font-geist-mono",
	subsets: ["latin"],
});

export const metadata = {
	title: "Brock Tomlinson",
	description:
		"The protfolio site for Brock Tomlinson, it has some blogs, my personal preferences all skill set",
};

export default function RootLayout({ children }) {
	return (
		<html lang="en">
			{/* className={`${geistSans.variable} ${geistMono.variable}`} */}
			<body>
				<Navbar />
				<main>{children}</main>
				<Ender />
			</body>
		</html>
	);
}
