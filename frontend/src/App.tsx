// 路由
import { BrowserRouter, Routes, Route, Link } from "react-router-dom";

// 网页
import Home from "@/pages/Home";
import ResultsPage from "@/pages/ResultsPage";
import NotFound from "@/pages/NotFound";
import StatePreview from "@/pages/StatePreview";
import RecipeDetail, {
  type Recipe,
  type RecipeListLoader,
} from "@/pages/RecipeDetail";

// 根据API文档的详细数据，用于前端测试
const loadTestRecipes: RecipeListLoader = (signal) =>
  new Promise((resolve, reject) => {
    if (signal.aborted) {
      reject(new DOMException("Request aborted", "AbortError"));
      return;
    }

    const recipe: Recipe = {
      id: "560",
      title: "爆炒腰花",
      description: "",
      ingredients: [
        "猪腰子",
        "茭白",
        "葱",
        "酱油",
        "盐",
        "料酒",
        "淀粉",
        "糖",
      ].map((name) => ({ name, amount: "" })),
      steps: [
        {
          number: 1,
          text: "腰花买一破二，把中间白色的地方去掉，外面的一层膜去掉。用花刀切好，用盐、淀粉、料酒腌制一下。茭白切丝备用",
          image_urls: [],
        },
        {
          number: 2,
          text: "起油锅放姜先爆炒腰花，放茭白，加酱油、糖、盐等调味",
          image_urls: [],
        },
        { number: 3, text: "盛起加葱就可以了", image_urls: [] },
      ],
      tips: "",
      cover_image_urls: [
        "https://i2.chuimg.com/a9ba4f287c8e11e591cde0db5512b208.jpg?imageView2/1/w/640/h/520/q/75/format/jpg",
      ],
      author: "小月紫",
      author_url: "https://mip.xiachufang.com/cook/100446/",
      url: "https://mip.xiachufang.com/recipe/560/",
    };
    const timeout = window.setTimeout(() => {
      signal.removeEventListener("abort", abort);
      resolve({ items: [recipe], total: 1 });
    }, 600);
    function abort() {
      window.clearTimeout(timeout);
      reject(new DOMException("Request aborted", "AbortError"));
    }
    signal.addEventListener("abort", abort, { once: true });
  });

// 路由
export default function App() {
  return (
    <BrowserRouter>
      <Routes>
        {/* 主页 */}
        <Route path="/" element={<Home />} />

        {/* 搜索页 */}
        <Route path="/results" element={<ResultsPage />} />

        {/* 详细页面 */}
        <Route path="/recipe/:id" element={<RecipeDetail />} />
        {import.meta.env.DEV && (
          <Route path="/dev/ui-states" element={<StatePreview />} />
        )}
        {import.meta.env.DEV && (
          <Route
            path="/dev/recipe/:id"
            element={<RecipeDetail loadRecipes={loadTestRecipes} />}
          />
        )}

        {/* 错误页面 */}
        <Route path="*" element={<NotFound />} />
      </Routes>
    </BrowserRouter>
  );
}
