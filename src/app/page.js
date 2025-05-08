"use client";

import Image from "next/image";
import Ender from "../components/footer/ender";
import "./home.css";
import Link from "next/link";

export default function Home() {
	return (
		<main>
			<div id="home-intro">
				<h1> Hi I'm Brock </h1>
				<p>
					a fourth year Software Engineering Student specializing in full-stack
					development with a strong focus on backend technologies. I am
					developing the language of how to design, develop, and create programs
					that are to industry standards and reasonably efficent. I bring the
					lessons learned from each project I have completed, learning from the
					mistakes I have made and bringing improved versions forward into the
					next project.
				</p>
				<p>
					As of writing this I intend to bring the knowledge learned from my
					time at university in Software Engineering onto a Baja SAE car, where
					we can collect data remotely and graph data for instantaneous and
					future analysis, during vechile operation.
				</p>
				<p>
					I grew up in a small ski town where, I started learning about
					programming, from of course Minecraft, where I thought the application
					of this was so futuristic and downright cool that I knew I wanted to
					persure it further. While living there I spend a majority of my time
					outside of school swimming competitively, where I ranked top 10 in BC.
					Along with swimming I spend a lot of time volunteering with
					fundraising events and coaching the local Special Olympics swim team.
				</p>
				<p>
					I advore problem solving and building cool stuff, I'm happy to jump in
					and get started!
					<Link href="ContactMe">Let's create something great together!</Link>
				</p>
			</div>
		</main>
	);
}
