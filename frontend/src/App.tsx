import { BrowserRouter, Routes, Route, Link } from 'react-router-dom'
import { Button } from "@/components/ui/button"

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