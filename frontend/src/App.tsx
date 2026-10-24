// Router
import { BrowserRouter, Routes, Route } from "react-router-dom";

// Pages
import Home from "@/pages/Home";
import ResultsPage from "@/pages/ResultsPage";
import NotFound from "@/pages/NotFound";
import RecipeDetail from "@/pages/RecipeDetail";

// Router
export default function App() {
  return (
    <BrowserRouter>
      <Routes>
        {/* Main */}
        <Route path="/" element={<Home />} />

        {/* Results page */}
        <Route path="/results" element={<ResultsPage />} />

        {/* Detail page */}
        <Route path="/recipe/:id" element={<RecipeDetail />} />

        {/* Not found */}
        <Route path="*" element={<NotFound />} />
      </Routes>
    </BrowserRouter>
  );
}
