import { ChefHat, House, UtensilsCrossed } from 'lucide-react'
import { Link } from 'react-router-dom'
import { Button } from '@/components/ui/button'

export default function NotFound() {
  return (
    <div className="flex min-h-svh flex-col bg-background px-6 py-8 font-sans text-foreground">
      <header className="mx-auto w-full max-w-5xl">
        <Link to="/" className="inline-flex items-center gap-2 rounded-md font-heading text-2xl font-semibold outline-none focus-visible:ring-3 focus-visible:ring-ring/50">
          <ChefHat aria-hidden="true" className="size-7 text-primary" />
          Recipe Finder
        </Link>
      </header>
      <main className="flex flex-1 flex-col items-center justify-center gap-6 py-16 text-center">
        <div className="flex size-24 items-center justify-center rounded-full bg-primary/10 text-primary">
          <UtensilsCrossed aria-hidden="true" className="size-10" />
        </div>
        <p className="rounded-full border border-border px-4 py-1.5 text-sm font-medium text-muted-foreground">
          404 · Page not found
        </p>
        <h1 className="max-w-2xl font-heading text-4xl font-semibold md:text-5xl">
          This page is off the menu
        </h1>
        <p className="max-w-md text-base text-muted-foreground">
          We couldn’t find the page you’re looking for. Head back home to find your next recipe.
        </p>
        <Button
          size="lg"
          nativeButton={false}
          render={<Link to="/" />}
          className="mt-2"
        >
          <House aria-hidden="true" className="size-4" />
          Back to home
        </Button>
      </main>
    </div>
  )
}
