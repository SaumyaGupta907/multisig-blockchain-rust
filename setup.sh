#!/bin/bash

echo "🦀 Rust Blockchain Project Setup & Verification"
echo "=============================================="
echo ""

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "📦 Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
else
    echo "✅ Rust is already installed"
fi

echo ""
echo "🔨 Building project..."
cargo build --release

if [ $? -eq 0 ]; then
    echo "✅ Build successful!"
else
    echo "❌ Build failed"
    exit 1
fi

echo ""
echo "🧪 Running tests..."
cargo test

if [ $? -eq 0 ]; then
    echo "✅ All tests passed!"
else
    echo "❌ Some tests failed"
    exit 1
fi

echo ""
echo "🚀 Running demo..."
cargo run

echo ""
echo "=============================================="
echo "✅ Setup Complete!"
echo ""
echo "📚 Next steps:"
echo "  1. Review README.md for technical details"
echo "  2. Study INTERVIEW_PREP.md for talking points"
echo "  3. Run 'cargo run' to see the demo again"
echo "  4. Run 'cargo test -- --nocapture' for detailed test output"
echo ""
echo "🎯 You're ready for the Fidelity interview!"
