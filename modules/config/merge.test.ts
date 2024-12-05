import { describe, expect, test } from 'bun:test';
import { mergeWithDefaults } from './merge.ts';  // Adjust the import based on your file structure

describe('mergeWithDefaults', () => {

    test('should assign values from source to target where target has undefined values', () => {
        const target = {prop1: undefined, prop2: 42};
        const source = {prop1: 'value1', prop2: 66};

        //@ts-ignore
        const result = mergeWithDefaults(target, source);
        //@ts-ignore
        expect(result).toEqual({prop1: 'value1', prop2: 42});
    });

    test('should assign values from source to target where target has null values', () => {
        const target = {prop1: null, prop2: 42};
        const source = {prop1: 'value1', prop2: 66};
        //@ts-ignore
        const result = mergeWithDefaults(target, source);
        //@ts-ignore
        expect(result).toEqual({prop1: 'value1', prop2: 42});
    });

    test('should not overwrite defined values in target', () => {
        const target = {prop1: 'existing', prop2: 42};
        const source = {prop1: 'newValue', prop2: 99};

        const result = mergeWithDefaults(target, source);
        expect(result).toEqual({prop1: 'existing', prop2: 42});
    });

    test('should add properties from source that do not exist in target', () => {
        const target = {prop1: 'existing'};
        const source = {prop2: 'added'};

        //@ts-ignore
        const result = mergeWithDefaults(target, source);
        //@ts-ignore
        expect(result).toEqual({prop1: 'existing', prop2: 'added'});
    });

    test('should handle empty target and source correctly', () => {
        const target = {};
        const source = {prop1: 'fromSource'};

        const result = mergeWithDefaults(target, source);
        expect(result).toEqual({prop1: 'fromSource'});
    });

    test('should handle target and source with differing properties', () => {
        const target = {prop1: 'targetValue', prop3: 'targetOnly'};
        const source = {prop2: 'sourceValue', prop3: null};

        //@ts-ignore
        const result = mergeWithDefaults(target, source);
        //@ts-ignore
        expect(result).toEqual({prop1: 'targetValue', prop2: 'sourceValue', prop3: 'targetOnly'});
    });

});