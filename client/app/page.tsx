"use client";

import { Button } from "@/components/ui/button";
import Test from "./test";

export default function Home() {
  return (
    <div className="flex min-h-screen flex-col items-center justify-center gap-5 bg-gray-dark text-white">
      <h1 className="text-2xl">Component Playground</h1>
      <Button
        className="bg-blurple hover:bg-blurple/80"
        onClick={() => console.log("Discord button clicked!")}
      >
        Discord Button
      </Button>
      <Test />
    </div>
  );
}