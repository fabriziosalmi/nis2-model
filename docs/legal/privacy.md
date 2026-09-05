# Privacy Policy

**Last updated:** 5 September 2026

## Data Controller

Fabrizio Salmi — [GitHub](https://github.com/fabriziosalmi)

## What Data Is Processed, and Why

Everything below is processed for one purpose: to answer your question about
NIS2 applicability in your own browser, and to keep the site available. There is
no other purpose — no analytics, no profiling, no advertising, and no
measurement of who you are or what you looked at.


### Client-Side Processing (No Server Transmission)

The nis2-model chatbot processes the following data **entirely in your browser**. No data is sent to any server operated by the project:

| Data | Purpose | Storage | Legal Basis (GDPR) |
|------|---------|---------|-------------------|
| Your questions (free text) | Matched against local Q&A dataset via BM25 search | In-memory only, cleared on page reload | Legitimate interest (Art. 6(1)(f)) |
| Numeric parameters (employees, revenue) | Real-time applicability assessment | In-memory only, cleared on page reload | Legitimate interest (Art. 6(1)(f)) |
| Browser language (`navigator.language`) | UI language auto-detection (IT/EN) | In-memory only | Legitimate interest (Art. 6(1)(f)) |
| Session state (visited categories) | Coverage tracking, follow-up suggestions | In-memory only, cleared on page reload | Legitimate interest (Art. 6(1)(f)) |

### Hosting Provider (GitHub Pages)

This site is hosted on [GitHub Pages](https://docs.github.com/en/pages/getting-started-with-github-pages/about-github-pages). GitHub may collect:

- IP addresses
- Browser user-agent strings
- Access timestamps

This data is processed by GitHub Inc. under their [Privacy Statement](https://docs.github.com/en/site-policy/privacy-policies/github-general-privacy-statement). GitHub acts as a data processor for hosting purposes.

**Transfer outside the EEA.** GitHub Inc. is established in the United States, so
delivering a page transfers technical data such as your IP address there. The
transfer is covered by GitHub's Data Processing Agreement and by the EU–US Data
Privacy Framework / Standard Contractual Clauses. There are no other transfers.

**Retention.** We keep no personal data. Nothing you type is persisted: it stays
in memory and is gone when you reload or close the page. Any technical logs held
by GitHub as hosting provider are transient and retained only as long as needed
for delivery and security.

## What Data Is NOT Collected

- **No cookies** are set by this application
- **No analytics** or tracking scripts are loaded
- **No personal data** is transmitted to any server operated by the project
- **No accounts** or registration are required
- **No data** is persisted after you close or reload the page

## REST API

If you deploy and use the REST API (`nis2-api` crate), company profile data (name, sector, employees, revenue) is processed **on your own server**. The API does not log or persist request data by default. Deployers are responsible for their own GDPR compliance.

## Your Rights (GDPR Art. 15-22)

Since all chatbot processing happens client-side with no server-side storage, there is no personal data held by the project to access, rectify, erase, or port. For data processed by GitHub Pages, exercise your rights directly with [GitHub](https://support.github.com/contact/privacy).

## Changes

This policy may be updated. Changes will be reflected in the "Last updated" date above.

## Lodging a Complaint

If you believe this processing infringes the GDPR, you may lodge a complaint with
a supervisory authority — for this controller that is the Italian **Garante per
la protezione dei dati personali**
([garanteprivacy.it](https://www.garanteprivacy.it/)) — or with the authority of
the EU member state where you live or work (Art. 77 GDPR).

## Contact

For any privacy request, write to **fabrizio.salmi@gmail.com**. You may also open
an issue on [GitHub](https://github.com/fabriziosalmi/nis2-model), though a public
issue is a poor place for anything you would rather not publish.
