import { BrowserRouter, Routes, Route, Link } from 'react-router-dom'

// Main page
function Home() {
  return (
    <div className="flex flex-col items-center justify-center min-h-screen bg-gray-50">
      <h1 className="text-4xl font-bold text-blue-600 mb-4">
        Hello Tailwind & React! 🎉
      </h1>
      <p className="text-gray-600 mb-8">如果这段文字居中且标题是蓝色的，说明 Tailwind 配置成功了。</p>
      
      <Link to="/about" className="px-6 py-2 bg-black text-white rounded-md hover:bg-gray-800 transition">
        Go to About Page
      </Link>
    </div>
  )
}

// Routing
export default function App() {
  return (
    <BrowserRouter>
      <Routes>
        <Route path="/" element={<Home />} />
      </Routes>
    </BrowserRouter>
  )
}