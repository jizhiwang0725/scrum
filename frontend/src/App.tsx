import { BrowserRouter, Routes, Route, Link } from 'react-router-dom'
import { Button } from "@/components/ui/button"
import NotFound from '@/pages/NotFound'
import StatePreview from '@/pages/StatePreview'
import RecipeDetail, { type Recipe, type RecipeListLoader } from '@/pages/RecipeDetail'

// Development-only test API using the response example in docs/API.md.
const loadTestRecipes: RecipeListLoader = (signal) => new Promise((resolve, reject) => {
  if (signal.aborted) {
    reject(new DOMException('Request aborted', 'AbortError'))
    return
  }

  const recipe: Recipe = {
    id: '560',
    title: '爆炒腰花',
    description: '',
    ingredients: ['猪腰子', '茭白', '葱', '酱油', '盐', '料酒', '淀粉', '糖'].map((name) => ({ name, amount: '' })),
    steps: [
      { number: 1, text: '腰花买一破二，把中间白色的地方去掉，外面的一层膜去掉。用花刀切好，用盐、淀粉、料酒腌制一下。茭白切丝备用', image_urls: [] },
      { number: 2, text: '起油锅放姜先爆炒腰花，放茭白，加酱油、糖、盐等调味', image_urls: [] },
      { number: 3, text: '盛起加葱就可以了', image_urls: [] },
    ],
    tips: '',
    cover_image_urls: ['https://i2.chuimg.com/a9ba4f287c8e11e591cde0db5512b208.jpg?imageView2/1/w/640/h/520/q/75/format/jpg'],
    author: '小月紫',
    author_url: 'https://mip.xiachufang.com/cook/100446/',
    url: 'https://mip.xiachufang.com/recipe/560/',
  }
  const timeout = window.setTimeout(() => {
    signal.removeEventListener('abort', abort)
    resolve({ items: [recipe], total: 1 })
  }, 600)
  function abort() {
    window.clearTimeout(timeout)
    reject(new DOMException('Request aborted', 'AbortError'))
  }
  signal.addEventListener('abort', abort, { once: true })
})

// Main page
function Home() {
  return (
    <div className="font-heading min-h-screen bg-background text-foreground flex flex-col items-center justify-center">
      <h1 className="text-9xl">
        Hello Tailwind & React! 🎉
      </h1>
      
      <p className="text-foreground">
        Hello
      </p>
      
      <Button>
        <Link to="/about">
          Go to About Page
        </Link>
      </Button>

      {import.meta.env.DEV && (
        <Button className="mt-4" nativeButton={false} render={<Link to="/dev/recipe/560" />}>
          测试菜谱详情页
        </Button>
      )}

    </div>
  )
}

// Routing
export default function App() {
  return (
    <BrowserRouter>
      <Routes>
        <Route path="/" element={<Home />} />
        <Route path="/recipe/:id" element={<RecipeDetail />} />
        {import.meta.env.DEV && (
          <Route path="/dev/ui-states" element={<StatePreview />} />
        )}
        {import.meta.env.DEV && (
          <Route path="/dev/recipe/:id" element={<RecipeDetail loadRecipes={loadTestRecipes} />} />
        )}
        <Route path="*" element={<NotFound />} />
      </Routes>
    </BrowserRouter>
  )
}
