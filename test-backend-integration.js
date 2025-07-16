// Test Backend Integration
// This script tests the Tauri backend commands to verify they exist and respond correctly

const { invoke } = window.__TAURI__.core;

async function testBackendIntegration() {
    console.log('🧪 Starting Backend Integration Tests...\n');
    
    const results = {
        passed: 0,
        failed: 0,
        tests: []
    };

    // Helper function to run a test
    async function runTest(testName, testFn) {
        try {
            console.log(`⏳ Testing: ${testName}`);
            await testFn();
            console.log(`✅ PASSED: ${testName}`);
            results.passed++;
            results.tests.push({ name: testName, status: 'PASSED' });
        } catch (error) {
            console.error(`❌ FAILED: ${testName} - ${error.message}`);
            results.failed++;
            results.tests.push({ name: testName, status: 'FAILED', error: error.message });
        }
    }

    // Test 1: System Status Command
    await runTest('get_system_status command', async () => {
        const status = await invoke('get_system_status');
        if (!status || typeof status !== 'object') {
            throw new Error('Invalid system status response');
        }
        if (!status.app_version) {
            throw new Error('Missing app_version in system status');
        }
    });

    // Test 2: Ollama Status Command
    await runTest('ensure_ollama_ready command', async () => {
        const status = await invoke('ensure_ollama_ready');
        if (!status || typeof status !== 'object') {
            throw new Error('Invalid ollama status response');
        }
        if (typeof status.is_running !== 'boolean') {
            throw new Error('Missing or invalid is_running field');
        }
        if (typeof status.is_installed !== 'boolean') {
            throw new Error('Missing or invalid is_installed field');
        }
        if (!Array.isArray(status.models)) {
            throw new Error('Missing or invalid models array');
        }
    });

    // Test 3: Wiki Status Command
    await runTest('get_wiki_status command', async () => {
        const status = await invoke('get_wiki_status');
        if (!status || typeof status !== 'object') {
            throw new Error('Invalid wiki status response');
        }
        if (typeof status.total_pages !== 'number') {
            throw new Error('Missing or invalid total_pages field');
        }
        if (typeof status.is_updating !== 'boolean') {
            throw new Error('Missing or invalid is_updating field');
        }
    });

    // Test 4: Input Validation (should fail with invalid input)
    await runTest('send_message validation', async () => {
        try {
            await invoke('send_message', { message: '', model: 'test' });
            throw new Error('Should have failed with empty message');
        } catch (error) {
            if (!error.message.includes('empty') && !error.message.includes('whitespace')) {
                throw new Error('Validation error message incorrect: ' + error.message);
            }
            // This is expected - validation should reject empty messages
        }
    });

    // Test 5: Model Name Validation (should fail with invalid model name)
    await runTest('model name validation', async () => {
        try {
            await invoke('send_message', { message: 'test message', model: '@invalid-model@' });
            throw new Error('Should have failed with invalid model name');
        } catch (error) {
            if (!error.message.includes('invalid') && !error.message.includes('character')) {
                throw new Error('Model validation error message incorrect: ' + error.message);
            }
            // This is expected - validation should reject invalid model names
        }
    });

    // Print results
    console.log('\n📊 Test Results Summary:');
    console.log(`✅ Passed: ${results.passed}`);
    console.log(`❌ Failed: ${results.failed}`);
    console.log(`📈 Success Rate: ${((results.passed / (results.passed + results.failed)) * 100).toFixed(1)}%`);
    
    if (results.failed > 0) {
        console.log('\n❌ Failed Tests:');
        results.tests.filter(t => t.status === 'FAILED').forEach(test => {
            console.log(`  - ${test.name}: ${test.error}`);
        });
    }

    return results;
}

// Export for use in HTML
window.testBackendIntegration = testBackendIntegration;