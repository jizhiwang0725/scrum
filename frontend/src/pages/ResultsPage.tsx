import { useEffect, useState } from "react";
import { useSearchParams, useNavigate } from "react-router-dom";
import mockRecipes from "@/data/recipes.json";

// Components
import { Search, ChevronLeft } from "lucide-react"; // Arrow sign for "back" button
import { Card, CardHeader, CardTitle, CardContent } from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";

// Fake data structure
export type Recipe = {
  id: string;
  title: string;
  author: string;
  cover_image_urls: string[];
  stats_text: string;
  description: string;
};

export default function ResultsPage() {
  // Takes the keyword after "?" in the URL"
  const [searchParams, setSearchParams] = useSearchParams();
  const keyword = searchParams.get("keyword") || "";

  // Event emitter: notify App.tsx which new room (page) to join.
  const navigate = useNavigate(); // For navigation

  // Data filtering - need to be replace when the backend is completed
  const allRecipes = mockRecipes as Recipe[];

  const filteredResults = allRecipes.filter((recipe) => {
    if (!keyword) return true;
    return recipe.title.toLowerCase().includes(keyword.toLowerCase());
  });

  // Search box, takes the information inputted by the user
  const [inputValue, setInputValue] = useState(keyword);

  // Sync search state with URL query parameters
  useEffect(() => {
    setInputValue(keyword);
  }, [keyword]);

  // Realtime update URL while typing
  const handleInputChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const newValue = e.target.value;
    setInputValue(newValue);
    if (newValue.trim()) {
      setSearchParams({ keyword: newValue.trim() });
    } else {
      setSearchParams({});
    }
  };

  const handleSearch = (e: React.SubmitEvent<HTMLFormElement>) => {
    e.preventDefault();
  };

  return (
    <div className="min-h-screen py-10">
      <div className="container mx-auto max-w-6xl px-4 md:px-6">
        {/* ================= Search box ================= */}
        <div className="mb-14 flex w-full justify-center">
          <div className="flex w-full items-center gap-3 md:gap-4">
            {/* Back button */}
            <Button
              variant="back"
              onClick={() => navigate("/")} // Back to main
              className="h-14 py-4 md:h-16"
            >
              <ChevronLeft className="mr-1 h-4 w-4 md:h-5 md:w-5" />
              Back
            </Button>

            {/* Search form */}
            <form
              onSubmit={handleSearch}
              className="group relative flex h-14 w-full items-center"
            >
              {/* Search icon */}
              <Search className="group-focus-within: primary pointer-events-none absolute left-5 z-10 h-14 w-6 text-gray-400 transition-colors" />

              {/* Input box */}
              <Input
                type="search"
                placeholder="Search for pasta, vegan, keto..."
                value={inputValue}
                onChange={handleInputChange}
                className="h-14 rounded-2xl border-transparent bg-white/80 pr-2 pl-12 text-base shadow-[0_8px_30px_rgb(0,0,0,0.06)] backdrop-blur-sm transition-all duration-300 outline-none hover:shadow-[0_8px_30px_rgb(249,115,22,0.1)] focus-visible:ring-2 focus-visible:ring-orange-400/50 md:h-16 md:rounded-3xl md:pr-3 md:pl-14 md:text-lg"
              />
            </form>
          </div>
        </div>

        {/* ================= Page title ================= */}
        <div className="mb-8 flex items-end justify-between border-b pb-4">
          <h1 className="min-w-0 flex-1 truncate px-3 text-3xl tracking-tight text-gray-800">
            {keyword ? (
              <>
                Search results for:{" "}
                <span className="text-orange-500">{keyword}</span>
              </>
            ) : (
              "All Recipes"
            )}
          </h1>
          <span className="rounded-full border bg-white px-3 py-1 text-sm font-medium text-gray-500 shadow-sm">
            {filteredResults.length} recipe(s)
          </span>
        </div>

        {/* ================= Results list ================= */}
        {filteredResults.length > 0 ? (
          <div className="grid grid-cols-1 gap-8 px-3 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4">
            {filteredResults.map((recipe) => (
              <Card
                key={recipe.id}

                onClick={() => navigate(`/recipe/${recipe.id}`)}

                className="group flex h-full cursor-pointer flex-col overflow-hidden rounded-3xl border-transparent bg-white shadow-[0_4px_20px_rgb(0,0,0,0.04)] transition-all duration-300 hover:shadow-[0_8px_30px_rgb(249,115,22,0.12)]"
              >
                {/* Image container */}
                <div className="relative h-52 overflow-hidden">
                  {recipe.cover_image_urls.length > 0 && (
                    <img
                      src={recipe.cover_image_urls[0]}
                      alt={recipe.title}
                      className="h-full w-full object-cover transition-transform duration-500 group-hover:scale-105"
                    />
                  )}
                </div>

                <CardHeader className="px-6 pt-6 pb-2">
                  <CardTitle className="line-clamp-1 text-xl font-bold text-gray-800">
                    {recipe.title}
                  </CardTitle>
                  <p className="mt-1 text-xs font-medium text-gray-400">
                    Author: {recipe.author}
                  </p>
                </CardHeader>

                <CardContent className="flex flex-1 flex-col justify-between px-6 pb-6">
                  <p className="mb-4 line-clamp-2 text-sm leading-relaxed text-gray-500">
                    {recipe.description ||
                      "A delicious recipe you must try today. Click to explore the cooking steps!"}
                  </p>

                  {/* Cutting line */}
                  <div className="mt-auto flex items-center border-t border-gray-100 pt-4 text-xs font-semibold text-gray-400">
                    <span className="truncate">{recipe.stats_text}</span>
                  </div>
                </CardContent>
              </Card>
            ))}
          </div>
        ) : (
          /* ================= Null status ================= */
          <div className="mt-8 rounded-3xl border border-gray-50 bg-white py-24 text-center shadow-[0_4px_20px_rgb(0,0,0,0.02)]">
            <div className="mb-4 text-4xl">🔍</div>
            <h3 className="mb-2 text-xl font-bold text-gray-800">
              Oops! No recipes found
            </h3>
            <p className="break-all text-gray-500">
              We couldn't find anything matching "{keyword}".
            </p>
            <button
              onClick={() => {
                setSearchParams({});
                setInputValue("");
              }}
              className="mt-6 font-semibold text-orange-500 underline underline-offset-4 hover:text-orange-600"
            >
              Clear search and view all
            </button>
          </div>
        )}
      </div>
    </div>
  );
}
