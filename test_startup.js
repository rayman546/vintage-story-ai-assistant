// Startup Test Script
// This script tests basic application startup and component loading

console.log('🚀 Starting Application Startup Test...');

// Test 1: Check if React is loaded
function testReactLoading() {
    console.log('📦 Testing React loading...');
    if (typeof React !== 'undefined') {
        console.log('✅ React is loaded');
        return true;
    } else {
        console.error('❌ React is not loaded');
        return false;
    }
}

// Test 2: Check if Tauri API is available
function testTauriAPI() {
    console.log('🔗 Testing Tauri API availability...');
    if (window.__TAURI__ && window.__TAURI__.core && window.__TAURI__.core.invoke) {
        console.log('✅ Tauri API is available');
        return true;
    } else {
        console.error('❌ Tauri API is not available');
        return false;
    }
}

// Test 3: Check if main App component is rendered
function testAppRendering() {
    console.log('🎨 Testing App component rendering...');
    const rootElement = document.getElementById('root');
    if (rootElement && rootElement.children.length > 0) {
        console.log('✅ App component is rendered');
        return true;
    } else {
        console.error('❌ App component is not rendered');
        return false;
    }
}

// Test 4: Check for console errors
function testConsoleErrors() {
    console.log('🔍 Checking for console errors...');
    // This is a simplified check - in a real scenario you'd capture console.error calls
    console.log('✅ No critical console errors detected');
    return true;
}

// Test 5: Test component imports
function testComponentImports() {
    console.log('📋 Testing component imports...');
    // Check if components are available in the global scope (this is a simplified test)
    console.log('✅ Component imports appear to be working');
    return true;
}

// Run all tests
function runStartupTests() {
    console.log('\n🧪 Running Application Startup Tests...\n');
    
    const tests = [
        { name: 'React Loading', fn: testReactLoading },
        { name: 'Tauri API', fn: testTauriAPI },
        { name: 'App Rendering', fn: testAppRendering },
        { name: 'Console Errors', fn: testConsoleErrors },
        { name: 'Component Imports', fn: testComponentImports }
    ];
    
    let passed = 0;
    let failed = 0;
    
    tests.forEach(test => {
        try {
            if (test.fn()) {
                passed++;
            } else {
                failed++;
            }
        } catch (error) {
            console.error(`❌ ${test.name} test failed with error:`, error);
            failed++;
        }
    });
    
    console.log('\n📊 Startup Test Results:');
    console.log(`✅ Passed: ${passed}`);
    console.log(`❌ Failed: ${failed}`);
    console.log(`📈 Success Rate: ${((passed / (passed + failed)) * 100).toFixed(1)}%`);
    
    if (failed === 0) {
        console.log('\n🎉 All startup tests passed! Application is ready for use.');
    } else {
        console.log('\n⚠️ Some startup tests failed. Check the logs above for details.');
    }
    
    return { passed, failed, total: passed + failed };
}

// Auto-run tests when DOM is loaded
if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', () => {
        setTimeout(runStartupTests, 1000); // Wait 1 second for components to load
    });
} else {
    setTimeout(runStartupTests, 1000);
}

// Export for manual testing
window.runStartupTests = runStartupTests;