import { useId, type ReactNode } from 'react'
import { CircleAlert, LoaderCircle, RefreshCw } from 'lucide-react'
import { Button } from '@/components/ui/button'
import { cn } from '@/lib/utils'

// Define customizable error content, retry behavior, and styling.
export interface ErrorStateProps {
  message?: ReactNode
  onRetry: () => void
  title?: string
  retryLabel?: string
  retryingLabel?: string
  isRetrying?: boolean
  className?: string
}

// Display an error message with a retry button.
// The parent component manages the retry operation and its loading state.
export function ErrorState({
  message = 'We could not load this content. Please try again.',
  onRetry,
  title = 'Something went wrong',
  retryLabel = 'Try again',
  retryingLabel = 'Retrying...',
  isRetrying = false,
  className,
}: ErrorStateProps) {
  // Generate a unique ID to associate the alert with its heading.
  const titleId = useId()

  return (
    // Use the alert role to announce dynamically displayed errors to assistive technologies.
    <section
      role="alert"
      aria-labelledby={titleId}
      className={cn(
        'flex flex-col items-center justify-center gap-4 px-6 py-12 text-center font-sans',
        className,
      )}
    >
      {/* Display a decorative warning icon using the error color palette. */}
      <div className="flex size-16 items-center justify-center rounded-full bg-destructive/10 text-destructive">
        <CircleAlert aria-hidden="true" className="size-8" />
      </div>

      {/* Provide the alert's accessible name through its heading. */}
      <h2 id={titleId} className="font-heading text-2xl font-semibold text-foreground">
        {title}
      </h2>

      {/* Explain the failure and suggest how the user can proceed. */}
      <div className="max-w-md text-base break-words text-muted-foreground">
        {message}
      </div>

      {/* Disable repeated clicks and expose the busy state while retrying. */}
      <Button
        type="button"
        onClick={onRetry}
        disabled={isRetrying}
        aria-busy={isRetrying}
        className="mt-2"
      >
        {/* Show a spinner during retries and respect reduced-motion preferences. */}
        {isRetrying ? (
          <LoaderCircle aria-hidden="true" className="size-4 animate-spin motion-reduce:animate-none" />
        ) : (
          <RefreshCw aria-hidden="true" className="size-4" />
        )}

        {/* Update the button label to reflect the current retry state. */}
        {isRetrying ? retryingLabel : retryLabel}
      </Button>
    </section>
  )
}