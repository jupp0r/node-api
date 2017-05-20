const nt = require('./node-api');
const expect = require('chai').expect;

describe("node-api", function() {
    it("can return objects", function() {
        expect(nt.returns_objects()).to.deep.equal({'foo': 'hello', bar: 42});
    });
});
