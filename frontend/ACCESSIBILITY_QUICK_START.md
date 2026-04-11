# Accessibility Quick Start Guide

Get started with accessibility improvements in 5 minutes.

## 🚀 Quick Setup (5 minutes)

### 1. Install Dependencies

```bash
cd frontend
npm install
```

This installs all accessibility testing tools.

### 2. Run Your First Accessibility Check

```bash
# Check for accessibility issues in your code
npm run lint:a11y

# Run accessibility tests
npm run test:a11y
```

### 3. Install Browser Extensions

- **Chrome/Edge:** [axe DevTools](https://chrome.google.com/webstore/detail/axe-devtools-web-accessib/lhdoppojpmngadmnindnejefpokejbdd)
- **Firefox:** [WAVE](https://addons.mozilla.org/en-US/firefox/addon/wave-accessibility-tool/)

### 4. Test Your First Page

1. Open your app in the browser
2. Open DevTools (F12)
3. Click the "axe DevTools" tab
4. Click "Scan ALL of my page"
5. Review and fix issues

## 📋 Top 5 Quick Wins

### 1. Add Alt Text to Images

```tsx
// ❌ Bad
<img src="/logo.png" />

// ✅ Good
<img src="/logo.png" alt="Stellar Deepdive Logo" />

// ✅ Decorative images
<img src="/decoration.png" alt="" aria-hidden="true" />
```

### 2. Associate Labels with Inputs

```tsx
// ❌ Bad
<div>Email</div>
<input type="email" />

// ✅ Good
<label htmlFor="email">Email</label>
<input id="email" type="email" />
```

### 3. Add ARIA Labels to Icon Buttons

```tsx
// ❌ Bad
<button onClick={handleClose}>
  <X />
</button>

// ✅ Good
<button onClick={handleClose} aria-label="Close dialog">
  <X aria-hidden="true" />
</button>
```

### 4. Make Keyboard Navigation Work

```tsx
// ❌ Bad - div is not keyboard accessible
<div onClick={handleClick}>Click me</div>

// ✅ Good - button is keyboard accessible
<button onClick={handleClick}>Click me</button>
```

### 5. Add Skip Navigation

Already implemented! Just make sure your main content has `id="main-content"`:

```tsx
<main id="main-content">
  {/* Your content */}
</main>
```

## 🎯 Common Patterns

### Accessible Button

```tsx
import { Button } from '@/components/ui/button';

// Text button
<Button>Save Changes</Button>

// Icon button
<Button aria-label="Delete item">
  <Trash2 aria-hidden="true" />
</Button>

// Loading button
<Button loading loadingText="Saving...">
  Save
</Button>
```

### Accessible Form

```tsx
<form onSubmit={handleSubmit}>
  <div>
    <label htmlFor="name">
      Name <span aria-label="required">*</span>
    </label>
    <input
      id="name"
      type="text"
      required
      aria-required="true"
      aria-invalid={!!errors.name}
      aria-describedby={errors.name ? "name-error" : undefined}
    />
    {errors.name && (
      <p id="name-error" role="alert" className="text-red-500">
        {errors.name}
      </p>
    )}
  </div>
  
  <button type="submit">Submit</button>
</form>
```

### Accessible Modal

```tsx
<div
  role="dialog"
  aria-modal="true"
  aria-labelledby="dialog-title"
  aria-describedby="dialog-description"
>
  <h2 id="dialog-title">Confirm Action</h2>
  <p id="dialog-description">Are you sure?</p>
  
  <button onClick={handleClose} aria-label="Close dialog">
    <X aria-hidden="true" />
  </button>
  
  <div>
    <button onClick={handleConfirm}>Confirm</button>
    <button onClick={handleClose}>Cancel</button>
  </div>
</div>
```

### Accessible Loading State

```tsx
// ❌ Bad
<div>{loading && <Loader2 className="animate-spin" />}</div>

// ✅ Good
<div role="status" aria-live="polite">
  {loading && (
    <>
      <Loader2 className="animate-spin" aria-hidden="true" />
      <span className="sr-only">Loading...</span>
    </>
  )}
</div>
```

## 🧪 Testing Checklist

Before committing code, check:

- [ ] Can I navigate with Tab key?
- [ ] Are focus indicators visible?
- [ ] Do all images have alt text?
- [ ] Do all inputs have labels?
- [ ] Do icon buttons have aria-label?
- [ ] Does `npm run lint:a11y` pass?
- [ ] Does axe DevTools show 0 violations?

## 🆘 Common Issues & Fixes

### Issue: "Form elements must have labels"

```tsx
// Fix: Add label with htmlFor
<label htmlFor="email">Email</label>
<input id="email" type="email" />
```

### Issue: "Buttons must have discernible text"

```tsx
// Fix: Add aria-label for icon buttons
<button aria-label="Close">
  <X aria-hidden="true" />
</button>
```

### Issue: "Images must have alt text"

```tsx
// Fix: Add alt attribute
<img src="/chart.png" alt="Sales chart showing 25% increase" />
```

### Issue: "Elements must have sufficient color contrast"

```tsx
// Fix: Use CSS variables with proper contrast
<p className="text-foreground">Text with good contrast</p>
```

### Issue: "Interactive elements must be keyboard accessible"

```tsx
// Fix: Use button instead of div
<button onClick={handleClick}>Click me</button>
```

## 📚 Learn More

- **Full Documentation:** See `ACCESSIBILITY_README.md`
- **Implementation Guide:** See `ACCESSIBILITY_IMPLEMENTATION_GUIDE.md`
- **Checklist:** See `ACCESSIBILITY_CHECKLIST.md`
- **Audit Report:** See `ACCESSIBILITY_AUDIT.md`

## 🎓 5-Minute Training Videos

1. [WebAIM: Keyboard Accessibility](https://webaim.org/articles/keyboard/)
2. [Google: Introduction to ARIA](https://developers.google.com/web/fundamentals/accessibility/semantics-aria)
3. [MDN: ARIA Basics](https://developer.mozilla.org/en-US/docs/Learn/Accessibility/WAI-ARIA_basics)

## 💡 Pro Tips

1. **Use semantic HTML first** - `<button>` instead of `<div onClick>`
2. **Test with keyboard only** - Unplug your mouse and try navigating
3. **Use browser DevTools** - axe DevTools catches most issues
4. **Think about screen readers** - Would this make sense if you couldn't see it?
5. **Check color contrast** - Use WebAIM Contrast Checker

## 🚨 Red Flags

Watch out for these common mistakes:

- ❌ `<div onClick>` instead of `<button>`
- ❌ Images without alt text
- ❌ Inputs without labels
- ❌ Icon buttons without aria-label
- ❌ Low contrast text
- ❌ Keyboard traps in modals
- ❌ Missing focus indicators

## ✅ Green Flags

Good accessibility practices:

- ✅ Semantic HTML (`<button>`, `<nav>`, `<main>`)
- ✅ All images have alt text
- ✅ All inputs have labels
- ✅ Keyboard navigation works
- ✅ Focus indicators visible
- ✅ ARIA used appropriately
- ✅ Color contrast meets WCAG AA

## 🎯 Your First Task

Pick one of these to start:

1. **Easy:** Add alt text to all images in one component
2. **Medium:** Fix form labels in one form
3. **Advanced:** Make one chart keyboard accessible

## 📞 Get Help

- **Questions?** Check `ACCESSIBILITY_README.md`
- **Stuck?** Ask in the team channel
- **Found a bug?** Create an issue with `accessibility` label

---

**Remember:** Accessibility is not a feature, it's a requirement. Every user deserves equal access to our platform.

**Last updated:** February 23, 2026
