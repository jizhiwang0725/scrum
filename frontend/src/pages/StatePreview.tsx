import { useEffect, useState } from 'react'
import { ArrowLeft, Moon, Sun } from 'lucide-react'
import { Link } from 'react-router-dom'
import { EmptyState } from '@/components/common/EmptyState'
import { ErrorState } from '@/components/common/ErrorState'
import { Loading } from '@/components/common/Loading'
import { Button } from '@/components/ui/button'
import { cn } from '@/lib/utils'

export default function StatePreview() {
  const [isDark, setIsDark] = useState(false)
  const [isRetrying, setIsRetrying] = useState(false)
  const [hasRetried, setHasRetried] = useState(false)

  useEffect(() => {
    if (!isRetrying) return

    const timeout = window.setTimeout(() => {
      setIsRetrying(false)
      setHasRetried(true)
    }, 900)

    return () => window.clearTimeout(timeout)
  }, [isRetrying])

  return (
    <div className={cn(isDark && 'dark')}>
      <main className="min-h-svh bg-background px-6 py-10 font-sans text-foreground md:px-10">
        <div className="mx-auto max-w-6xl space-y-8">
          <header className="flex flex-wrap items-center justify-between gap-4">
            <div className="space-y-3">
              <Link to="/" className="inline-flex items-center gap-2 rounded-md text-sm text-muted-foreground outline-none hover:text-foreground focus-visible:ring-3 focus-visible:ring-ring/50">
                <ArrowLeft aria-hidden="true" className="size-4" />
                Back to home
              </Link>
              <h1 className="font-heading text-4xl font-semibold md:text-5xl">UI states</h1>
              <p className="text-base text-muted-foreground">Development preview of the shared status components.</p>
            </div>
            <Button variant="outline" type="button" onClick={() => setIsDark(!isDark)}>
              {isDark ? <Sun aria-hidden="true" className="size-4" /> : <Moon aria-hidden="true" className="size-4" />}
              {isDark ? 'Switch to light theme' : 'Switch to dark theme'}
            </Button>
          </header>

          <div className="grid gap-6 lg:grid-cols-3">
            <div className="rounded-xl border border-border bg-card text-card-foreground">
              <p className="border-b border-border px-5 py-3 text-sm font-medium">Loading</p>
              <Loading message="Finding something delicious..." className="min-h-80" />
            </div>
            <div className="rounded-xl border border-border bg-card text-card-foreground">
              <p className="border-b border-border px-5 py-3 text-sm font-medium">Empty state</p>
              <EmptyState
                title="No recipes found"
                message="We couldn’t find any recipes matching your ingredients. Try a different combination."
                className="min-h-80"
              />
            </div>
            <div className="rounded-xl border border-border bg-card text-card-foreground">
              <p className="border-b border-border px-5 py-3 text-sm font-medium">Error & retry</p>
              {hasRetried ? (
                <EmptyState
                  title="Request completed"
                  message="The retry succeeded. There are no recipes to display in this preview."
                  action={<Button type="button" variant="outline" onClick={() => setHasRetried(false)}>Reset error</Button>}
                  className="min-h-80"
                />
              ) : (
                <ErrorState
                  message="The recipe service is unavailable. Click Try again to simulate a successful retry."
                  onRetry={() => setIsRetrying(true)}
                  isRetrying={isRetrying}
                  className="min-h-80"
                />
              )}
            </div>
          </div>

          <Button variant="outline" nativeButton={false} render={<Link to="/__preview-not-found" />}>
            Preview 404 page
          </Button>
        </div>
      </main>
    </div>
  )
}
