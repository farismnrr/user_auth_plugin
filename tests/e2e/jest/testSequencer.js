const Sequencer = require("@jest/test-sequencer").default;

class CustomSequencer extends Sequencer {
  sort(tests) {
    // Sort tests by directory name (1_pre_test, 2_auth_test, 3_tenant_test, 4_user_test)
    // Then by filename within each directory
    const sortedTests = tests.sort((testA, testB) => {
      const pathA = testA.path;
      const pathB = testB.path;

      // Extract directory and filename
      const getDirAndFile = (path) => {
        const parts = path.split("/");
        const filename = parts[parts.length - 1];
        const dirname = parts[parts.length - 2];
        return { dirname, filename };
      };

      const a = getDirAndFile(pathA);
      const b = getDirAndFile(pathB);

      // Compare directory names first (1_pre_test < 2_auth_test < 3_tenant_test < 4_user_test)
      if (a.dirname !== b.dirname) {
        return a.dirname.localeCompare(b.dirname);
      }

      // Within same directory, compare filenames (1a < 1b < 2a < 2b, etc.)
      return a.filename.localeCompare(b.filename);
    });

    return sortedTests;
  }
}

module.exports = CustomSequencer;
