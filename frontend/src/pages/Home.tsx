import { useState } from "react";

// Router
import { useNavigate } from "react-router-dom";

// Components
import { Search } from "lucide-react";
import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";

// Main page title
import bgImage from "@/image/Title.png";

export default function Home() {
  const [query, setQuery] = useState("");
  const navigate = useNavigate();

  // Search logic
  const handleSearch = (e: React.SyntheticEvent<HTMLFormElement>) => {
    e.preventDefault();
    if (!query.trim()) return;
    navigate(`/results?keyword=${encodeURIComponent(query)}`);
  };

  return (
    <div className="relative flex min-h-screen flex-col items-center overflow-hidden pt-[10vh] md:pt-[12vh]">
      {/* Blurred colour background */}
      <div className="absolute top-10 left-1/4 h-72 w-72 rounded-full bg-orange-300 opacity-30 mix-blend-multiply blur-[100px] filter" />
      <div className="absolute top-40 right-1/4 h-72 w-72 rounded-full bg-yellow-200 opacity-30 mix-blend-multiply blur-[100px] filter" />

      {/* Title image */}
      <img
        src={bgImage}
        alt="Recipe Finder"
        className="relative z-10 h-62.5 w-[90%] max-w-125 object-cover object-center drop-shadow-sm md:h-87.5 md:max-w-175"
      />

      {/* Text */}
      <p className="relative z-10 -mt-25 text-sm font-medium tracking-wide text-gray-500 md:-mt-30 md:text-base">
        What are you craving today? Discover thousands of recipes.
      </p>

      {/* Search */}
      <form
        onSubmit={handleSearch}
        className="group relative z-20 mx-auto mt-6 flex w-[90%] max-w-125 items-center md:mt-8 md:max-w-2xl"
      >
        {/* Search box decoration - the colour of the border changes when focused */}
        <Search className="pointer-events-none absolute left-5 z-10 h-6 w-6 text-gray-400 transition-colors group-focus-within:text-orange-500" />

        {/* Input box */}
        <Input
          type="search"
          value={query}
          onChange={(e) => setQuery(e.target.value)}
          // target is like self in python, it refers to the component itself.
          // This connects the changes on the browser to the actual code

          placeholder="Search for pasta, vegan, keto..."
          className="h-14 rounded-2xl border-transparent bg-white/80 pr-28 pl-12 text-base shadow-[0_8px_30px_rgb(0,0,0,0.06)] backdrop-blur-sm transition-all duration-300 outline-none hover:shadow-[0_8px_30px_rgb(249,115,22,0.1)] focus-visible:ring-2 focus-visible:ring-orange-400/50 md:h-16 md:rounded-3xl md:pr-36 md:pl-14 md:text-lg"
        />

        {/* Search button */}
        <Button
          type="submit"
          className="absolute right-2 h-10 rounded-xl border-none bg-linear-to-r from-orange-400 to-orange-500 px-6 font-semibold text-white shadow-lg shadow-orange-500/30 transition-all duration-300 hover:scale-105 hover:from-orange-500 hover:to-orange-600 hover:shadow-orange-500/50 md:right-2.5 md:h-12 md:rounded-2xl md:px-8"
        >
          Search
        </Button>
      </form>
    </div>
  );
}
