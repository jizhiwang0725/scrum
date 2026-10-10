import { useId, type ReactNode } from 'react'
import { CircleAlert, LoaderCircle, RefreshCw } from 'lucide-react'
import { Button } from '@/components/ui/button'
import { cn } from '@/lib/utils'

export interface ErrorStateProps {
  message?: ReactNode
  onRetry: () => void
  title?: string
  retryLabel?: string
  retryingLabel?: string
  isRetrying?: boolean
  className?: string
}

export function ErrorState({
  message = 'We could not load this content. Please try again.',
  onRetry,
  title = 'Something went wrong',
  retryLabel = 'Try again',
  retryingLabel = 'Retrying...',
  isRetrying = false,
  className,
}: ErrorStateProps) {
  const titleId = useId()

  return (
    <section
      role="alert"
      aria-labelledby={titleId}
      className={cn(
        'flex flex-col items-center justify-center gap-4 px-6 py-12 text-center font-sans',
        className,
      )}
    >
      <div className="flex size-16 items-center justify-center rounded-full bg-destructive/10 text-destructive">
        <CircleAlert aria-hidden="true" className="size-8" />
      </div>
      <h2 id={titleId} className="font-heading text-2xl font-semibold text-foreground">
        {title}
      </h2>
      <div className="max-w-md text-base break-words text-muted-foreground">
        {message}
      </div>
      <Button
        type="button"
        onClick={onRetry}
        disabled={isRetrying}
        aria-busy={isRetrying}
        className="mt-2"
      >
        {isRetrying ? (
          <LoaderCircle aria-hidden="true" className="size-4 animate-spin motion-reduce:animate-none" />
        ) : (
          <RefreshCw aria-hidden="true" className="size-4" />
        )}
        {isRetrying ? retryingLabel : retryLabel}
      </Button>
    </section>
  )
}
