import Link from "next/link";
import "./navbar.css";

export default function Navbar() {
	return (
		<div id="navbar">
			<Link href="/">Home</Link>
			<Link href="/projects">Projects</Link>
			<Link href="/blogs/0">Blogs</Link>
			<Link href="/Contact">Contact</Link>
		</div>
	);
}
