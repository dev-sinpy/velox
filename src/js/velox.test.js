// Basic tests for velox, which would help to find any problems
// in a particular functionality of the app

let _TESTS = {

    // test notification functionality
    testNotification: async function() {
        try {
            let res = await window.__VELOX__.showNotification("test", "this is a test notification", 3000);
            return true;
        } catch (err) {
            return false;
        }
    },

    // test window functionality
    testSetTitle: async function() {
        try {
            let res = await window.__VELOX__.window.setTitle("title");
            return true;
        } catch (err) {
            return false;
        }
    },

    testSetTransparent: async function() {
        try {
            let res = await window.__VELOX__.window.setTransparent(true);
            return true;
        } catch (err) {
            return false;
        }
    },

    testSetFullscreen: async function() {
        try {
            let res = await window.__VELOX__.window.setFullscreen(true);
            return true;
        } catch (err) {
            return false;
        }
    },

    // test subprocess functionality
    testExec: async function() {
        try {
            let res = await window.__VELOX__.subprocess.exec('echo "hello world"', ".", false);
            return true;
        } catch (err) {
            return false;
        }
    },

    // test filesystem functionality
    testCreateDir: async function() {
        try {
            let res = await window.__VELOX__.fs.createDir("./test");
            return true;
        } catch (err) {
            return false;
        }
    },
    testCreateFile: async function() {
        try {
            let res = await window.__VELOX__.fs.createFile("./test/demo.txt");
            return true;
        } catch (err) {
            return false;
        }
    },
    testReadTextFile: async function() {
        try {
            let res = await window.__VELOX__.fs.readTextFile("./test/demo.txt");
            return true;
        } catch (err) {
            return false;
        }
    },
    testReadDir: async function() {
        try {
            let res = await window.__VELOX__.fs.readDir("./test/");
            return true;
        } catch (err) {
            return false;
        }
    },
    testCopyFile: async function() {
        try {
            let res = await window.__VELOX__.fs.copyFile("./test/demo.txt", "./test/demo.txt");
            return true;
        } catch (err) {
            return false;
        }
    },
    testRenameFile: async function() {
        try {
            let res = await window.__VELOX__.fs.renameFile("./test/demo.txt", "./test/demo.txt");
            return true;
        } catch (err) {
            return false;
        }
    },
    testSaveFile: async function() {
        try {
            let utf8String = Array.from(new TextEncoder().encode("This is a test."));
            let res = await window.__VELOX__.fs.saveFile("./test/new.txt", utf8String, "w");
            return true;
        } catch (err) {
            return false;
        }
    },
    testRemoveFile: async function() {
        try {
            let res = await window.__VELOX__.fs.removeFile("./test/demo.txt")
            return true;
        } catch (err) {
            return false;
        }
    },
    testRemoveDir: async function() {
        try {
            let res = await window.__VELOX__.fs.removeDir("./test/");
            return true;
        } catch (err) {
            return false;
        }
    },

};


async function runTests() {
    let testResult = {};

    for (const [key, value] of Object.entries(_TESTS)) {
        testResult[key] = await value() ? "PASSED" : "FAILED";
    }

    console.table(testResult)

    return testResult;
}