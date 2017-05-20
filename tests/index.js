const nt = require('./node-api');
const expect = require('chai').expect;

describe("node-api", function() {
    it("can return objects", function() {
        expect(nt.returns_objects()).to.deep.equal({'foo': 'hello', bar: 42});
    });
    it("can return strings", function() {
        expect(nt.returns_strings()).to.equal("returned_string");
    });
    it("can return numbers", function() {
        expect(nt.returns_numbers()).to.equal(42);
    });
    it("can return booleans", function() {
        expect(nt.returns_booleans()).to.equal(true);
    });
});
