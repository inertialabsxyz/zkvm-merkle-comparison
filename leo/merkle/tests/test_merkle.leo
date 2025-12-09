// The 'test_merkle' test program.
import merkle_final_3.aleo;
program test_merkle.aleo {
    @test
    script test_has() {
        let result: field = merkle_final_3.aleo/test_hash(1field, 2field);
        assert_eq(result, 2799419348328628267772397258993881637724900331651364255144693240264270898381field);
    }

    @test
    script test_manual_proof() {
        let result: field = merkle_final_3.aleo/test_hash(2101455173202777488396631090253606380122940466565147449004343386925113994488field, 6825249408765352434972294941645202423199277711885737506547093127203208032814field);
        let result_1: field = merkle_final_3.aleo/test_hash(1453422445951123753151106172778857958705957519805227827612723076529186482115field, result);
        assert_eq(result_1, 3048425985363766419703157494923397905930052110958124269811445958420343108200field);
    }

    @test
    script test_proof() {
        let proof = MerkleProof {
            root: 3048425985363766419703157494923397905930052110958124269811445958420343108200field,
            index: 2u32,
            leaf: 2101455173202777488396631090253606380122940466565147449004343386925113994488field,
            siblings: [
              6825249408765352434972294941645202423199277711885737506547093127203208032814field,
              1453422445951123753151106172778857958705957519805227827612723076529186482115field,
            ],
        };
        assert(merkle_final_3.aleo/main(proof));
    }

    @test
    transition do_nothing() {
//         let result: u32 = merkle.aleo/main(2u32, 3u32);
        assert_eq(0u32, 0u32);
    }

    @noupgrade
    async constructor() {}
}
