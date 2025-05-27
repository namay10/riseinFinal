# Review Documentation

This document outlines the steps to build, deploy, and test the Solana program, along with frontend integration.

## Prerequisites

- Rust and Cargo installed
- Solana CLI installed
- Anchor Framework installed
- Node.js and npm installed

## Building and Deploying the Program

1. Build the program:

```bash
anchor build
```

2. Deploy to local validator:

```bash
anchor deploy
```

3. Note the Program ID from the deployment output. It will look something like:

```bash
 Demo Program Id: 7KqpRwzkkeweW5jQoETyLzhvs9rcCj9dVQ1MnzudirsM
```

## Program ID Integration

1. Update the Program ID in frontend:
   - Open `frontend/src/constants/programId.ts`
   - Replace the existing program ID with your deployed program ID:

```typescript
export const PROGRAM_ID = "7KqpRwzkkeweW5jQoETyLzhvs9rcCj9dVQ1MnzudirsM";
```

## Testing

1. Run program tests:

```bash
anchor test
```

2. Run frontend tests:

```bash
cd frontend
npm test
```

## Frontend Setup

1. Navigate to frontend directory:

```bash
cd frontend
```

2. Install dependencies:

```bash
npm install
```

3. Start the development server:

```bash
npm run dev
```

## Common Issues and Solutions

1. Program ID Mismatch

   - Ensure the program ID is updated in both `Anchor.toml` and frontend code
   - Rebuild and redeploy if necessary

2. Build Errors

   - Check Rust version compatibility
   - Verify Anchor.toml configuration
   - Clean build artifacts: `anchor clean`

3. Deployment Issues
   - Ensure local validator is running
   - Check sufficient SOL balance
   - Verify program size limits

## Testing Files Location

- Program Tests: `programs/your-program/tests/`
- Frontend Tests: `frontend/src/tests/`
- Integration Tests: `tests/`

## Contact

For deployment or integration issues, please contact the project maintainers.
