import { House, UtensilsCrossed } from "lucide-react";
import { Link } from "react-router-dom";
import { Button } from "@/components/ui/button";

// Display a 404 page when the current URL does not match an available route.
export default function NotFound() {
  return (
    // Use a full-height layout with a header and a flexible main content area.
    <div className="bg-background text-foreground flex min-h-svh flex-col px-6 py-8 font-sans">
      {/* Center the error message and navigation button in the remaining space. */}
      <main className="flex flex-1 flex-col items-center justify-center gap-6 py-16 text-center">
        {/* Hide the decorative icon from assistive technologies. */}
        <div className="bg-primary/10 text-primary flex size-24 items-center justify-center rounded-full">
          <UtensilsCrossed aria-hidden="true" className="size-10" />
        </div>

        {/* Identify the page as a 404 error. */}
        <p className="border-border text-muted-foreground rounded-full border px-4 py-1.5 text-sm font-medium">
          404 · Page not found
        </p>

        <h1 className="font-heading max-w-2xl text-4xl font-semibold md:text-5xl">
          This page is off the menu
        </h1>
        <p className="text-muted-foreground max-w-md text-base">
          We couldn't find the page you're looking for. Head back home to find
          your next recipe.
        </p>

        {/* Render the button as a router link for client-side navigation home. */}
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
  );
}
