import { useId, type ReactNode } from 'react'
import { Search } from 'lucide-react'
import { cn } from '@/lib/utils'

// Define optional content and styling for the empty state.
// ReactNode allows text, elements, or other renderable content.
export interface EmptyStateProps {
  title?: string
  message?: ReactNode
  icon?: ReactNode
  action?: ReactNode
  className?: string
}

// Display a reusable empty state with default text and a search icon.
export function EmptyState({
  title = 'No results found',
  message = 'Try a different search or adjust your filters.',
  icon = <Search className="size-8" />,
  action,
  className,
}: EmptyStateProps) {
  // Generate a unique ID to associate this section with its heading.
  const titleId = useId()

  return (
    <section
      aria-labelledby={titleId}
      className={cn(
        'flex flex-col items-center justify-center gap-4 px-6 py-12 text-center font-sans',
        className,
      )}
    >
      {/* Hide the decorative icon from assistive technologies. */}
      <div
        aria-hidden="true"
        className="flex size-16 items-center justify-center rounded-full bg-primary/10 text-primary"
      >
        {icon}
      </div>

      {/* Provide the section's accessible name through its heading. */}
      <h2 id={titleId} className="font-heading text-2xl font-semibold text-foreground">
        {title}
      </h2>

      {/* Explain the empty state or suggest a next step. */}
      <div className="max-w-md text-base break-words text-muted-foreground">
        {message}
      </div>

      {/* Render optional actions, such as a button to reset search filters. */}
      {action && <div className="mt-2 flex flex-wrap justify-center gap-3">{action}</div>}
    </section>
  )
}