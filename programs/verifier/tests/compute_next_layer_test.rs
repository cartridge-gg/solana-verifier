use felt::Felt;
use stark::stark_proof::stark_verify::compute_next_layer::ComputeNextLayer;
use stark::swiftness::fri::types::FriLayerQuery;
use stark::swiftness::stark::types::FriVerifyData;
use utils::{BidirectionalStack, Scheduler};
use verifier::state::BidirectionalStackAccount;

fn setup_fri_verify_data(
    queries: Vec<FriLayerQuery>,
    coset_size: Felt,
    eval_point: Felt,
    sibling_witness: Vec<Felt>,
) -> FriVerifyData {
    let mut fri_verify_data = FriVerifyData::default();

    // Set up queries
    for query in queries {
        fri_verify_data.working_queries.push(query);
    }

    // Set up sibling witness in sibling_witness field
    for witness in sibling_witness {
        fri_verify_data.sibling_witness.push(witness);
    }

    // Set parameters
    fri_verify_data.coset_size = coset_size;
    fri_verify_data.eval_point = eval_point;

    fri_verify_data
}

fn run_compute_next_layer_test(
    initial_queries: Vec<FriLayerQuery>,
    sibling_witness: Vec<Felt>,
    coset_size: Felt,
    eval_point: Felt,
    expected_verify_indices: Vec<Felt>,
    expected_verify_y_values: Vec<Felt>,
    expected_next_queries: Vec<FriLayerQuery>,
) {
    let mut stack = BidirectionalStackAccount::default();

    // Setup FriVerifyData
    let fri_verify_data = setup_fri_verify_data(
        initial_queries.clone(),
        coset_size,
        eval_point,
        sibling_witness.clone(),
    );

    // Store in cache
    stack.store_in_cache(&fri_verify_data);

    // Create and run ComputeNextLayer task using scheduler like other tests
    let task = ComputeNextLayer::new();
    stack.push_task(task);

    while !stack.is_empty_back() {
        stack.execute();
    }

    // Check that task completed successfully with correct results
    let fri_verify_data_after_execution = stack.borrow_from_cache::<FriVerifyData>();

    // Check verify_indices (working_indices) - should match expected
    println!(
        "✅ Working indices after execution: {}",
        fri_verify_data_after_execution.working_indices.len()
    );
    assert_eq!(
        fri_verify_data_after_execution.working_indices.len(),
        expected_verify_indices.len()
    );
    for i in 0..expected_verify_indices.len() {
        if let Some(actual_index) = fri_verify_data_after_execution.working_indices.get(i) {
            assert_eq!(
                *actual_index, expected_verify_indices[i],
                "Verify index {} mismatch: expected {}, got {}",
                i, expected_verify_indices[i], *actual_index
            );
        }
    }

    // working_y_values should contain verify_y_values (coset elements)
    println!(
        "✅ Working y_values after execution: {}",
        fri_verify_data_after_execution.working_y_values.len()
    );

    assert_eq!(
        fri_verify_data_after_execution.working_y_values.len(),
        expected_verify_y_values.len()
    );
    for i in 0..expected_verify_y_values.len() {
        if let Some(actual_y_value) = fri_verify_data_after_execution.working_y_values.get(i) {
            assert_eq!(
                *actual_y_value, expected_verify_y_values[i],
                "Verify y_value {} mismatch: expected {}, got {}",
                i, expected_verify_y_values[i], *actual_y_value
            );
        }
    }

    // working_elements should contain coset elements for FriFormula
    println!(
        "✅ Working elements after execution: {}",
        fri_verify_data_after_execution.working_elements.len()
    );

    // next_queries should contain next_queries (result from FriFormula)
    println!(
        "✅ Next queries after execution: {}",
        fri_verify_data_after_execution.next_queries.len()
    );

    // Debug: print next queries
    for i in 0..fri_verify_data_after_execution.next_queries.len() {
        if let Some(query) = fri_verify_data_after_execution.next_queries.get(i) {
            println!(
                "   next_query[{}]: index={}, y_value={}, x_inv_value={}",
                i, query.index, query.y_value, query.x_inv_value
            );
        }
    }

    // working_queries should be consumed (empty or contain remaining queries for next cosets)
    println!(
        "✅ Working queries after execution: {}",
        fri_verify_data_after_execution.working_queries.len()
    );

    assert_eq!(
        fri_verify_data_after_execution.next_queries.len(),
        expected_next_queries.len()
    );
    for i in 0..expected_next_queries.len() {
        if let Some(actual_next_query) = fri_verify_data_after_execution.next_queries.get(i) {
            assert_eq!(
                *actual_next_query, expected_next_queries[i],
                "Next query {} mismatch: expected {:?}, got {:?}",
                i, expected_next_queries[i], *actual_next_query
            );
        }
    }

    // Verify parameters are still correct
    assert_eq!(fri_verify_data_after_execution.coset_size, coset_size);
    assert_eq!(fri_verify_data_after_execution.eval_point, eval_point);

    println!("✅ ComputeNextLayer execution test passed!");
    println!("   - Started with {} queries", initial_queries.len());
    println!(
        "   - Started with {} sibling witness values",
        sibling_witness.len()
    );
    println!("   - Coset size: {}", coset_size);
    println!(
        "   - Verify indices match expected: {:?}",
        expected_verify_indices
    );
}

#[test]
fn test_next_layer1() {
    let queries = vec![FriLayerQuery {
        index: Felt::from_dec_str("19").unwrap(),
        y_value: Felt::from_dec_str(
            "3009640132648008425771663319959959262486220333099664427328058765768842449250",
        )
        .unwrap(),
        x_inv_value: Felt::from_dec_str(
            "54553547539894122403827046258314152559223416585866015792288871627619859008",
        )
        .unwrap(),
    }];

    let sibling_witness = vec![Felt::from_dec_str(
        "1123008634785466227765787920403783137942925653310144335875674694591276473192",
    )
    .unwrap()];

    let coset_size = Felt::from(2);
    let eval_point = Felt::from_dec_str(
        "2443479172752326919986485065418726216145528469562787664528210275186695390471",
    )
    .unwrap();

    let expected_verify_indices = vec![Felt::from(9)];
    let expected_verify_y_values = vec![
        Felt::from_dec_str(
            "1123008634785466227765787920403783137942925653310144335875674694591276473192",
        )
        .unwrap(), // sibling witness
        Felt::from_dec_str(
            "3009640132648008425771663319959959262486220333099664427328058765768842449250",
        )
        .unwrap(), // query y_value
    ];
    let expected_next_queries = vec![FriLayerQuery {
        index: Felt::from(9),
        y_value: Felt::from_dec_str(
            "2185991974161515735589370697017442079762615282775874094325294026252936541583",
        )
        .unwrap(),
        x_inv_value: Felt::from_dec_str(
            "1697286867097833219836294983908733326167501326977247172278435262719557593504",
        )
        .unwrap(),
    }];

    run_compute_next_layer_test(
        queries,
        sibling_witness,
        coset_size,
        eval_point,
        expected_verify_indices,
        expected_verify_y_values,
        expected_next_queries,
    );
}

#[test]
fn test_next_layer2() {
    let queries = vec![FriLayerQuery {
        index: Felt::from(17),
        y_value: Felt::from_dec_str(
            "2759623090142790690855098103901650892631267048724128835098128315349419136695",
        )
        .unwrap(),
        x_inv_value: Felt::from_dec_str(
            "108072969398534483334507045980403840714691619221758030761568305841993401905",
        )
        .unwrap(),
    }];

    let sibling_witness = vec![
        Felt::from_dec_str(
            "522165234549733937327724288257767164912068272740875848443122078655609268368",
        )
        .unwrap(),
        Felt::from_dec_str(
            "62652482029070998724738900051279388447617017224333293756545714721568375652",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2828526802429981059567341974328690206758626003655409262291038410023069024668",
        )
        .unwrap(),
    ];

    let coset_size = Felt::from(4);
    let eval_point = Felt::from_dec_str(
        "1982585344331058375026005306471531766707340420809531025829328268083147325627",
    )
    .unwrap();

    let expected_verify_indices = vec![Felt::from(4)];
    // For coset_size=4, we expect 4 y_values in coset order (16,17,18,19):
    // Position 0 (index 16): sibling witness, Position 1 (index 17): query, Position 2-3: sibling witness
    let expected_verify_y_values = vec![
        Felt::from_dec_str(
            "522165234549733937327724288257767164912068272740875848443122078655609268368",
        )
        .unwrap(), // sibling witness (position 0, index 16)
        Felt::from_dec_str(
            "2759623090142790690855098103901650892631267048724128835098128315349419136695",
        )
        .unwrap(), // query y_value (position 1, index 17)
        Felt::from_dec_str(
            "62652482029070998724738900051279388447617017224333293756545714721568375652",
        )
        .unwrap(), // sibling witness (position 2, index 18)
        Felt::from_dec_str(
            "2828526802429981059567341974328690206758626003655409262291038410023069024668",
        )
        .unwrap(), // sibling witness (position 3, index 19)
    ];
    let expected_next_queries = vec![FriLayerQuery {
        index: Felt::from(4),
        y_value: Felt::from_dec_str(
            "1077998794348284354598652343293562698049252500211550192464629080328316673698",
        )
        .unwrap(),
        x_inv_value: Felt::from_dec_str(
            "2633238744713263728019330547123200367032740465948237927517365901877698129956",
        )
        .unwrap(),
    }];

    run_compute_next_layer_test(
        queries,
        sibling_witness,
        coset_size,
        eval_point,
        expected_verify_indices,
        expected_verify_y_values,
        expected_next_queries,
    );
}

#[test]
fn test_next_layer5() {
    let queries = vec![
        FriLayerQuery {
            index: Felt::from(0),
            y_value: Felt::from_dec_str(
                "746591562936821030835715747889843595163396575226181906015101280529866617133",
            )
            .unwrap(),
            x_inv_value: Felt::from_dec_str(
                "2839890473743673236434239057700432645437719700778360715637471771392219034326",
            )
            .unwrap(),
        },
        FriLayerQuery {
            index: Felt::from(9),
            y_value: Felt::from_dec_str(
                "432934865743208963457432082990566669321350335050040579267198873031828688172",
            )
            .unwrap(),
            x_inv_value: Felt::from_dec_str(
                "2944384488565703107562664215590005833661704434344191167631969776808729729163",
            )
            .unwrap(),
        },
    ];

    let sibling_witness = vec![
        Felt::from_dec_str(
            "2207292640602765987605417203853603264378518893482237233873998269956850458932",
        )
        .unwrap(),
        Felt::from_dec_str(
            "288897229768628159275046633516552015154393884429636644820405109361062428074",
        )
        .unwrap(),
    ];

    let coset_size = Felt::from(2);
    let eval_point = Felt::from_dec_str(
        "2185465603412249558871549627667757930155426034864563482170756257080076225958",
    )
    .unwrap();

    let expected_verify_indices = vec![Felt::from(0), Felt::from(4)];
    // For 2 queries with coset_size=2, we expect 4 y_values total (2 cosets * 2 elements each)
    let expected_verify_y_values = vec![
        Felt::from_dec_str(
            "746591562936821030835715747889843595163396575226181906015101280529866617133",
        )
        .unwrap(), // query 1 y_value (index 0)
        Felt::from_dec_str(
            "2207292640602765987605417203853603264378518893482237233873998269956850458932",
        )
        .unwrap(), // sibling witness 1
        Felt::from_dec_str(
            "288897229768628159275046633516552015154393884429636644820405109361062428074",
        )
        .unwrap(), // sibling witness 2
        Felt::from_dec_str(
            "432934865743208963457432082990566669321350335050040579267198873031828688172",
        )
        .unwrap(), // query 2 y_value (index 9)
    ];
    let expected_next_queries = vec![
        FriLayerQuery {
            index: Felt::from(0),
            y_value: Felt::from_dec_str(
                "1960029025666243333485194840332621335802675978036586078469434126072698208300",
            )
            .unwrap(),
            x_inv_value: Felt::from_dec_str(
                "3009148876954521410390276225740133816385634755252667189136222755512973018079",
            )
            .unwrap(),
        },
        FriLayerQuery {
            index: Felt::from(4),
            y_value: Felt::from_dec_str(
                "501258974496117438650836974499861760052921694630156096214408435425138801490",
            )
            .unwrap(),
            x_inv_value: Felt::from_dec_str(
                "1715280836225574219633496220661969295029181757403817732538849012006942881738",
            )
            .unwrap(),
        },
    ];

    run_compute_next_layer_test(
        queries,
        sibling_witness,
        coset_size,
        eval_point,
        expected_verify_indices,
        expected_verify_y_values,
        expected_next_queries,
    );
}

#[test]
fn test_next_layer6() {
    let queries = vec![
        FriLayerQuery {
            index: Felt::from(5),
            y_value: Felt::from_dec_str(
                "1096892199810651185152195112986319610579001690328932435764292779877576915121",
            )
            .unwrap(),
            x_inv_value: Felt::from_dec_str(
                "3279544897416438008064089022127475409847685010480359106267514969650981832082",
            )
            .unwrap(),
        },
        FriLayerQuery {
            index: Felt::from(19),
            y_value: Felt::from_dec_str(
                "1641855356491365833554692237319212189801903876432464673866647716719043041465",
            )
            .unwrap(),
            x_inv_value: Felt::from_dec_str(
                "2293250085048948415112043044736231472430743506786581498262287561751356762014",
            )
            .unwrap(),
        },
    ];

    let sibling_witness = vec![
        Felt::from_dec_str(
            "804399642855128535722354372618913360973130781887293769086134887025287828450",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1987605673743133585231493245929792399393204021630711397835563456074854788026",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1233031324852744699980974111822491085958694791795843758111112233603894427238",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1810994839619816704905050921987428341202461226961859918732361321793970914107",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1892124182925365936167405294153806740491710255246363205322154303233181243718",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1337441648818842689794955442935277091168720593363441772344593529889075124973",
        )
        .unwrap(),
    ];

    let coset_size = Felt::from(4);
    let eval_point = Felt::from_dec_str(
        "2167335350971787138507326388801205881447440912920817749862011161467677463940",
    )
    .unwrap();

    let expected_verify_indices = vec![Felt::from(1), Felt::from(4)];
    let expected_verify_y_values = vec![
        Felt::from_dec_str(
            "804399642855128535722354372618913360973130781887293769086134887025287828450",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1096892199810651185152195112986319610579001690328932435764292779877576915121",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1987605673743133585231493245929792399393204021630711397835563456074854788026",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1233031324852744699980974111822491085958694791795843758111112233603894427238",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1810994839619816704905050921987428341202461226961859918732361321793970914107",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1892124182925365936167405294153806740491710255246363205322154303233181243718",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1337441648818842689794955442935277091168720593363441772344593529889075124973",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1641855356491365833554692237319212189801903876432464673866647716719043041465",
        )
        .unwrap(),
    ];
    let expected_next_queries = vec![
        FriLayerQuery {
            index: Felt::from(1),
            y_value: Felt::from_dec_str(
                "1181712651684323748024837749719027006506698122447393260787788152426027849043",
            )
            .unwrap(),
            x_inv_value: Felt::from_dec_str(
                "1653115732351104701033693337462649968391138809812222376293178295401862046874",
            )
            .unwrap(),
        },
        FriLayerQuery {
            index: Felt::from(4),
            y_value: Felt::from_dec_str(
                "1268308829684608985195972079707177792876762551009734230680402124542318187648",
            )
            .unwrap(),
            x_inv_value: Felt::from_dec_str(
                "1164447905508868651636386487771626691241643134212100820768777787963076904518",
            )
            .unwrap(),
        },
    ];

    run_compute_next_layer_test(
        queries,
        sibling_witness,
        coset_size,
        eval_point,
        expected_verify_indices,
        expected_verify_y_values,
        expected_next_queries,
    );
}

#[test]
fn test_next_layer7() {
    let queries = vec![
        FriLayerQuery {
            index: Felt::from(7),
            y_value: Felt::from_dec_str(
                "3075517878492540103040166847866926446865777778640852962190520816632339596145",
            )
            .unwrap(),
            x_inv_value: Felt::from_dec_str(
                "608694432915418906087773663985135750833888921733668764639429291959306418477",
            )
            .unwrap(),
        },
        FriLayerQuery {
            index: Felt::from(27),
            y_value: Felt::from_dec_str(
                "131983826742021719455910211379866988384811584925288184480272187873636838108",
            )
            .unwrap(),
            x_inv_value: Felt::from_dec_str(
                "1974198664155436145837620318999161049479840212206498623330601633289848275514",
            )
            .unwrap(),
        },
    ];

    let sibling_witness = vec![
        Felt::from_dec_str(
            "1381946771663536271686283445165923669245048036336685207435678635918790010415",
        )
        .unwrap(),
        Felt::from_dec_str(
            "987192345921964650774860449076993743666953560925616837171412357893248515453",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1894120995282262673257341015927038862976390070699798006337306824994818751092",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1538484798885111384061490573014877696930911320706092642348929689059341036045",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2831036501176131779443820825633947566967253654306554236912113528745500638916",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2795762391236757584469537519991014507421060627124928065640245513052288926693",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2962264866957003315891979312858498756904185604349911432280048675808497143212",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1274615238507625446045782756423380204591533819608003040326255896608966494630",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2433725008144059202602710302459177343649577363035970221093985127914132732545",
        )
        .unwrap(),
        Felt::from_dec_str(
            "3615760742796657913769142930324341926341661354106616058377178459338456365110",
        )
        .unwrap(),
        Felt::from_dec_str(
            "876491878808193133805417604582642436856178348784148668921521578336602541775",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1681379446663085402254280534212103014447726541607694761283737523967486068323",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2765034771912257979612046850994949532603486328163588398162583894664641390299",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2828844164871944409923044279501485001881689120102902225098373010416437899881",
        )
        .unwrap(),
    ];

    let coset_size = Felt::from(8);
    let eval_point = Felt::from_dec_str(
        "384372973063416357441038011066619352380639619775069754153768649812098431519",
    )
    .unwrap();

    let expected_verify_indices = vec![Felt::from(0), Felt::from(3)];
    let expected_verify_y_values = vec![
        Felt::from_dec_str(
            "1381946771663536271686283445165923669245048036336685207435678635918790010415",
        )
        .unwrap(),
        Felt::from_dec_str(
            "987192345921964650774860449076993743666953560925616837171412357893248515453",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1894120995282262673257341015927038862976390070699798006337306824994818751092",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1538484798885111384061490573014877696930911320706092642348929689059341036045",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2831036501176131779443820825633947566967253654306554236912113528745500638916",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2795762391236757584469537519991014507421060627124928065640245513052288926693",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2962264866957003315891979312858498756904185604349911432280048675808497143212",
        )
        .unwrap(),
        Felt::from_dec_str(
            "3075517878492540103040166847866926446865777778640852962190520816632339596145",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1274615238507625446045782756423380204591533819608003040326255896608966494630",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2433725008144059202602710302459177343649577363035970221093985127914132732545",
        )
        .unwrap(),
        Felt::from_dec_str(
            "3615760742796657913769142930324341926341661354106616058377178459338456365110",
        )
        .unwrap(),
        Felt::from_dec_str(
            "131983826742021719455910211379866988384811584925288184480272187873636838108",
        )
        .unwrap(),
        Felt::from_dec_str(
            "876491878808193133805417604582642436856178348784148668921521578336602541775",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1681379446663085402254280534212103014447726541607694761283737523967486068323",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2765034771912257979612046850994949532603486328163588398162583894664641390299",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2828844164871944409923044279501485001881689120102902225098373010416437899881",
        )
        .unwrap(),
    ];
    let expected_next_queries = vec![
        FriLayerQuery {
            index: Felt::from(0),
            y_value: Felt::from_dec_str(
                "3519382920442250662715268714283575071860170073120425261766286332519433463480",
            )
            .unwrap(),
            x_inv_value: Felt::from_dec_str(
                "266451709395649792495102217411295552361654455242989832116307089851296053314",
            )
            .unwrap(),
        },
        FriLayerQuery {
            index: Felt::from(3),
            y_value: Felt::from_dec_str(
                "3413008593626981122050040132687328477275093156117246104307650363140459009419",
            )
            .unwrap(),
            x_inv_value: Felt::from_dec_str(
                "1107290807486481817999551113742590485010425323783882499076376127930136554678",
            )
            .unwrap(),
        },
    ];

    run_compute_next_layer_test(
        queries,
        sibling_witness,
        coset_size,
        eval_point,
        expected_verify_indices,
        expected_verify_y_values,
        expected_next_queries,
    );
}

#[test]
fn test_next_layer8() {
    let queries = vec![
        FriLayerQuery {
            index: Felt::from(14),
            y_value: Felt::from_dec_str(
                "2812191291368465400252368036784849631295311044099688251864335547264648760929",
            )
            .unwrap(),
            x_inv_value: Felt::from_dec_str(
                "3327960025629984902906808973965115139132894388720619540209395295175202435223",
            )
            .unwrap(),
        },
        FriLayerQuery {
            index: Felt::from(28),
            y_value: Felt::from_dec_str(
                "945312106572582053105339715798467543580544183072055346899673660213540546488",
            )
            .unwrap(),
            x_inv_value: Felt::from_dec_str(
                "1982376188084661049306441397930389303876068734739596367142474139857967005184",
            )
            .unwrap(),
        },
    ];

    let sibling_witness = vec![
        Felt::from_dec_str(
            "2949257002824431099820924461178954292967579447203636252898226318072740386824",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2109770268208121078753001053403331765076608697772559113071033136250869504473",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2388297558783464615628997798028262325218009201379703050956169802493581047973",
        )
        .unwrap(),
        Felt::from_dec_str(
            "3525692657203325625152712276222448417400555150313331588381183925321722344670",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1770128582224668658932820753069082677343177339697504686745994727405384368826",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2277509151931760256130277827415848713946907170081107197539171861875808782840",
        )
        .unwrap(),
        Felt::from_dec_str(
            "553506533886672046972094815070428061413496218667260813272218476397542394126",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1325795902063165511059838253657321320005294114487993699345770044761244463890",
        )
        .unwrap(),
        Felt::from_dec_str(
            "669828035369528580542389024653781788264954760854641450329504420359621694341",
        )
        .unwrap(),
        Felt::from_dec_str(
            "3390712226968146726780043862598840971254869615115905720403199256814995219514",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1826681158268766190323480513465452190809042283119310757871839564743833787919",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1145304238778090805545988213888864245409497800124759254791897692806774757770",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1397068525986579081352897876232234437189969937826283879172500553676064940439",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2104875872195729864851606343399273464419019953918563072841629145823575557945",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1158938700019692875943299533882908302674841511080317290819608130684595336457",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2365573810001212351939469441049776991978930374259229467362964834664954075464",
        )
        .unwrap(),
        Felt::from_dec_str(
            "224204551411584072281213703508765114269364171685297633020108282783402412102",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1136885745908677333683443250622936914218130194199188396198410064853784245116",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2577202183647129179972590136619634609575314391523504634933521339842505024250",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1218879808821400330218160186881430073477575542748635158557783902419507248023",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1492583771642768717317176274379006337265687706897820888128354257084402032954",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1147051782978409736537631164196473096558884903070473339936475019730892606292",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1170923665353223025390644385475570698017370798971415401329877009135901058934",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2714965104434656612067886463451103798146458879447249983304400313485321454858",
        )
        .unwrap(),
        Felt::from_dec_str(
            "3029036750822909179098438143195941816494025920675957101572384069330532165665",
        )
        .unwrap(),
        Felt::from_dec_str(
            "18409245281450708426000590154856732233696909859426757887349330897284752834",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1903399152209210559959462040157108438124673941817349364218680564081568770298",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2260002643446022813142304377372681801652650136648130581924766564336949881263",
        )
        .unwrap(),
        Felt::from_dec_str(
            "3155765214842786052334914920203360592720185878259088994232602983562348781025",
        )
        .unwrap(),
        Felt::from_dec_str(
            "3485201017653558756685420512495162933189527567610822904102000763401993219662",
        )
        .unwrap(),
    ];

    let coset_size = Felt::from(16);
    let eval_point = Felt::from_dec_str(
        "52871414864247307776012409456547680662415039421837737271534775961443350705",
    )
    .unwrap();

    let expected_verify_indices = vec![Felt::from(0), Felt::from(1)];
    let expected_verify_y_values = vec![
        Felt::from_dec_str(
            "2949257002824431099820924461178954292967579447203636252898226318072740386824",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2109770268208121078753001053403331765076608697772559113071033136250869504473",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2388297558783464615628997798028262325218009201379703050956169802493581047973",
        )
        .unwrap(),
        Felt::from_dec_str(
            "3525692657203325625152712276222448417400555150313331588381183925321722344670",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1770128582224668658932820753069082677343177339697504686745994727405384368826",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2277509151931760256130277827415848713946907170081107197539171861875808782840",
        )
        .unwrap(),
        Felt::from_dec_str(
            "553506533886672046972094815070428061413496218667260813272218476397542394126",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1325795902063165511059838253657321320005294114487993699345770044761244463890",
        )
        .unwrap(),
        Felt::from_dec_str(
            "669828035369528580542389024653781788264954760854641450329504420359621694341",
        )
        .unwrap(),
        Felt::from_dec_str(
            "3390712226968146726780043862598840971254869615115905720403199256814995219514",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1826681158268766190323480513465452190809042283119310757871839564743833787919",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1145304238778090805545988213888864245409497800124759254791897692806774757770",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1397068525986579081352897876232234437189969937826283879172500553676064940439",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2104875872195729864851606343399273464419019953918563072841629145823575557945",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2812191291368465400252368036784849631295311044099688251864335547264648760929",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1158938700019692875943299533882908302674841511080317290819608130684595336457",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2365573810001212351939469441049776991978930374259229467362964834664954075464",
        )
        .unwrap(),
        Felt::from_dec_str(
            "224204551411584072281213703508765114269364171685297633020108282783402412102",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1136885745908677333683443250622936914218130194199188396198410064853784245116",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2577202183647129179972590136619634609575314391523504634933521339842505024250",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1218879808821400330218160186881430073477575542748635158557783902419507248023",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1492583771642768717317176274379006337265687706897820888128354257084402032954",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1147051782978409736537631164196473096558884903070473339936475019730892606292",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1170923665353223025390644385475570698017370798971415401329877009135901058934",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2714965104434656612067886463451103798146458879447249983304400313485321454858",
        )
        .unwrap(),
        Felt::from_dec_str(
            "3029036750822909179098438143195941816494025920675957101572384069330532165665",
        )
        .unwrap(),
        Felt::from_dec_str(
            "18409245281450708426000590154856732233696909859426757887349330897284752834",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1903399152209210559959462040157108438124673941817349364218680564081568770298",
        )
        .unwrap(),
        Felt::from_dec_str(
            "945312106572582053105339715798467543580544183072055346899673660213540546488",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2260002643446022813142304377372681801652650136648130581924766564336949881263",
        )
        .unwrap(),
        Felt::from_dec_str(
            "3155765214842786052334914920203360592720185878259088994232602983562348781025",
        )
        .unwrap(),
        Felt::from_dec_str(
            "3485201017653558756685420512495162933189527567610822904102000763401993219662",
        )
        .unwrap(),
    ];
    let expected_next_queries = vec![
        FriLayerQuery {
            index: Felt::from(0),
            y_value: Felt::from_dec_str(
                "2731422975982182882081741213504406076248280179242291132508067217003010322490",
            )
            .unwrap(),
            x_inv_value: Felt::from_dec_str(
                "3183581941728587291517385359778659637268454365590489450398239584747532037461",
            )
            .unwrap(),
        },
        FriLayerQuery {
            index: Felt::from(1),
            y_value: Felt::from_dec_str(
                "3228016798076912219844600739928191932167567203345640315464632263570697784005",
            )
            .unwrap(),
            x_inv_value: Felt::from_dec_str(
                "434920846937543922179937423316410468354652849741107249574852471388339983020",
            )
            .unwrap(),
        },
    ];

    run_compute_next_layer_test(
        queries,
        sibling_witness,
        coset_size,
        eval_point,
        expected_verify_indices,
        expected_verify_y_values,
        expected_next_queries,
    );
}

#[test]
fn test_next_layer9() {
    let queries = vec![
        FriLayerQuery {
            index: Felt::from(10),
            y_value: Felt::from_dec_str(
                "3301979551015115642776773522833072926462865561144560488920209857718769165321",
            )
            .unwrap(),
            x_inv_value: Felt::from_dec_str(
                "2447768544415408676335928447643487449650900913681401829424717259768986472741",
            )
            .unwrap(),
        },
        FriLayerQuery {
            index: Felt::from(28),
            y_value: Felt::from_dec_str(
                "260750552966409642499431036714653525048800120747768231293468725416260188565",
            )
            .unwrap(),
            x_inv_value: Felt::from_dec_str(
                "3563529224071196680374798485546840836554333985312746699506733128078602078641",
            )
            .unwrap(),
        },
        FriLayerQuery {
            index: Felt::from(29),
            y_value: Felt::from_dec_str(
                "1823311453035327473280509270574026429844738895506436406781130453908277513530",
            )
            .unwrap(),
            x_inv_value: Felt::from_dec_str(
                "54973564594934533322524297548229269068773230018850000466358928057269941840",
            )
            .unwrap(),
        },
    ];

    let sibling_witness = vec![Felt::from_dec_str(
        "3176532942878567276526900338617699157909586755721347090315239907293123933092",
    )
    .unwrap()];

    let coset_size = Felt::from(2);
    let eval_point = Felt::from_dec_str(
        "581747590720875793993157628416105910892274340828581484518962213541176928984",
    )
    .unwrap();

    let expected_verify_indices = vec![Felt::from(5), Felt::from(14)];
    let expected_verify_y_values = vec![
        Felt::from_dec_str(
            "3301979551015115642776773522833072926462865561144560488920209857718769165321",
        )
        .unwrap(),
        Felt::from_dec_str(
            "3176532942878567276526900338617699157909586755721347090315239907293123933092",
        )
        .unwrap(),
        Felt::from_dec_str(
            "260750552966409642499431036714653525048800120747768231293468725416260188565",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1823311453035327473280509270574026429844738895506436406781130453908277513530",
        )
        .unwrap(),
    ];
    let expected_next_queries = vec![
        FriLayerQuery {
            index: Felt::from(5),
            y_value: Felt::from_dec_str(
                "3275133527488003020272254384265923882734156428348855484827341126286781354142",
            )
            .unwrap(),
            x_inv_value: Felt::from_dec_str(
                "2615309274026883345480537437370039939583535394834355181916645336586036374461",
            )
            .unwrap(),
        },
        FriLayerQuery {
            index: Felt::from(14),
            y_value: Felt::from_dec_str(
                "2611221225089844146967129398570862707600738146407577663333768748170096584339",
            )
            .unwrap(),
            x_inv_value: Felt::from_dec_str(
                "1675506256769046824455009524724840487217963849380458734340688061164470257879",
            )
            .unwrap(),
        },
    ];

    run_compute_next_layer_test(
        queries,
        sibling_witness,
        coset_size,
        eval_point,
        expected_verify_indices,
        expected_verify_y_values,
        expected_next_queries,
    );
}

#[test]
fn test_next_layer10() {
    let queries = vec![
        FriLayerQuery {
            index: Felt::from(5),
            y_value: Felt::from_dec_str(
                "2302109513041081253191847726874632617695812467358434547038386746651006842380",
            )
            .unwrap(),
            x_inv_value: Felt::from_dec_str(
                "2637474160357003981779699526943464856611941641202539833636026322990388707333",
            )
            .unwrap(),
        },
        FriLayerQuery {
            index: Felt::from(13),
            y_value: Felt::from_dec_str(
                "2421144434847175762056784279666434264733454757381019920618274713336842008821",
            )
            .unwrap(),
            x_inv_value: Felt::from_dec_str(
                "3087662183041901605559596075538966726399103482477428404944198306175727302850",
            )
            .unwrap(),
        },
        FriLayerQuery {
            index: Felt::from(22),
            y_value: Felt::from_dec_str(
                "1359842048097095741378484164247564983223481090058269548624795715692521460139",
            )
            .unwrap(),
            x_inv_value: Felt::from_dec_str(
                "185282605276538732396669144005696787636433561792146384397375513028006583411",
            )
            .unwrap(),
        },
    ];

    let sibling_witness = vec![
        Felt::from_dec_str(
            "3354395884762193735774972945190699549697221284699421903066644213798392549402",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2286699216817078029209012960176095129558820839353213366512757899588073849784",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1807058458070289484044641026995619421272336800548938509837361700372957815942",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2613264906789519132611022617468068727030402878438205130898569384174203487539",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1731512844646762096812644150851970912816865225259598591671476871772161992367",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2809244789545431804862877174186922907097032744568338981087812219597365747358",
        )
        .unwrap(),
        Felt::from_dec_str(
            "3584511590009863696665267819675356849136392492253578574053929941069778312453",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2319520140453873363680878801076253902750880806504432226644283646473284304066",
        )
        .unwrap(),
        Felt::from_dec_str(
            "3403040452882679612874283723507253802829979450632690322212356915241676774155",
        )
        .unwrap(),
    ];

    let coset_size = Felt::from(4);
    let eval_point = Felt::from_dec_str(
        "842716418594610507165719701550798710728706432825705961415071015594374037783",
    )
    .unwrap();

    let expected_verify_indices = vec![Felt::from(1), Felt::from(3), Felt::from(5)];
    let expected_verify_y_values = vec![
        Felt::from_dec_str(
            "3354395884762193735774972945190699549697221284699421903066644213798392549402",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2302109513041081253191847726874632617695812467358434547038386746651006842380",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2286699216817078029209012960176095129558820839353213366512757899588073849784",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1807058458070289484044641026995619421272336800548938509837361700372957815942",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2613264906789519132611022617468068727030402878438205130898569384174203487539",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2421144434847175762056784279666434264733454757381019920618274713336842008821",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1731512844646762096812644150851970912816865225259598591671476871772161992367",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2809244789545431804862877174186922907097032744568338981087812219597365747358",
        )
        .unwrap(),
        Felt::from_dec_str(
            "3584511590009863696665267819675356849136392492253578574053929941069778312453",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2319520140453873363680878801076253902750880806504432226644283646473284304066",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1359842048097095741378484164247564983223481090058269548624795715692521460139",
        )
        .unwrap(),
        Felt::from_dec_str(
            "3403040452882679612874283723507253802829979450632690322212356915241676774155",
        )
        .unwrap(),
    ];
    let expected_next_queries = vec![
        FriLayerQuery {
            index: Felt::from(1),
            y_value: Felt::from_dec_str(
                "738283733844654320656593304955937814018241368923162576526604344171964798523",
            )
            .unwrap(),
            x_inv_value: Felt::from_dec_str(
                "473662685190715868134016058090152279944149415440903111962958772704259767472",
            )
            .unwrap(),
        },
        FriLayerQuery {
            index: Felt::from(3),
            y_value: Felt::from_dec_str(
                "969134386771919091675706269957962126061036556734351502037905523368628343664",
            )
            .unwrap(),
            x_inv_value: Felt::from_dec_str(
                "2176451726708652476016850844470876509775490912344006510026083913284976867308",
            )
            .unwrap(),
        },
        FriLayerQuery {
            index: Felt::from(5),
            y_value: Felt::from_dec_str(
                "2830771694438643841398564545385741940324054273824824277829790639217160649922",
            )
            .unwrap(),
            x_inv_value: Felt::from_dec_str(
                "2650304629722060934084881169738268161214211940849246175729038797083499024587",
            )
            .unwrap(),
        },
    ];

    run_compute_next_layer_test(
        queries,
        sibling_witness,
        coset_size,
        eval_point,
        expected_verify_indices,
        expected_verify_y_values,
        expected_next_queries,
    );
}

#[test]
fn test_next_layer11() {
    let queries = vec![
        FriLayerQuery {
            index: Felt::from(13),
            y_value: Felt::from_dec_str(
                "2012970895874678511212757784953971039384348922302933461566125878983543084374",
            )
            .unwrap(),
            x_inv_value: Felt::from_dec_str(
                "2973904717437488251876439033512297970752041300850017169618797599259028370719",
            )
            .unwrap(),
        },
        FriLayerQuery {
            index: Felt::from(27),
            y_value: Felt::from_dec_str(
                "931874247810412416165687814293757020658355126150232036748630683953363661570",
            )
            .unwrap(),
            x_inv_value: Felt::from_dec_str(
                "584004069736012205202772085193423872545377990373199751488130188652560879737",
            )
            .unwrap(),
        },
        FriLayerQuery {
            index: Felt::from(29),
            y_value: Felt::from_dec_str(
                "1239648551389298716869809742099762563284007154271743710729395207873328754095",
            )
            .unwrap(),
            x_inv_value: Felt::from_dec_str(
                "325989084577145299562436609653878858069349249438264163435312273435225996502",
            )
            .unwrap(),
        },
    ];

    let sibling_witness = vec![
        Felt::from_dec_str(
            "3132112378686662059596343294646770786600609238470988213583095363604854334582",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2461092923487112471463153426310804127458685592080740231075091398736912158061",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2998509969357752287758672740593580020870026463294812456319673321073443088049",
        )
        .unwrap(),
        Felt::from_dec_str(
            "555035695605537330949170393875380217638990235130626619093741676287189098559",
        )
        .unwrap(),
        Felt::from_dec_str(
            "72018361044517802373614267173646257173391950275613610500963681187632366491",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1356567855831745726697989920693118254887070010792127324660019977162220321572",
        )
        .unwrap(),
        Felt::from_dec_str(
            "3428903427156557890470369121770217952589955280129158495181620864445963618926",
        )
        .unwrap(),
        Felt::from_dec_str(
            "393083682526902362021532263518007213667960307630114380176309035415220154175",
        )
        .unwrap(),
        Felt::from_dec_str(
            "228421633383768634897088966132917335392548875057781107304223464921146078457",
        )
        .unwrap(),
        Felt::from_dec_str(
            "924492636261377530436326039939616684788498986913092833064848468110206034446",
        )
        .unwrap(),
        Felt::from_dec_str(
            "3215569544372428665026066156374535625549998775193271484978178438432055593239",
        )
        .unwrap(),
        Felt::from_dec_str(
            "89314992381896491523088980471634454610386898184975529899102140430198625686",
        )
        .unwrap(),
        Felt::from_dec_str(
            "3187985426015104063712461074067156374723555157039404691369397066532053540037",
        )
        .unwrap(),
    ];

    let coset_size = Felt::from(8);
    let eval_point = Felt::from_dec_str(
        "1891358614242486301056439669334692072313573517224693441784224259774268368657",
    )
    .unwrap();

    let expected_verify_indices = vec![Felt::from(1), Felt::from(3)];
    let expected_verify_y_values = vec![
        Felt::from_dec_str(
            "3132112378686662059596343294646770786600609238470988213583095363604854334582",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2461092923487112471463153426310804127458685592080740231075091398736912158061",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2998509969357752287758672740593580020870026463294812456319673321073443088049",
        )
        .unwrap(),
        Felt::from_dec_str(
            "555035695605537330949170393875380217638990235130626619093741676287189098559",
        )
        .unwrap(),
        Felt::from_dec_str(
            "72018361044517802373614267173646257173391950275613610500963681187632366491",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2012970895874678511212757784953971039384348922302933461566125878983543084374",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1356567855831745726697989920693118254887070010792127324660019977162220321572",
        )
        .unwrap(),
        Felt::from_dec_str(
            "3428903427156557890470369121770217952589955280129158495181620864445963618926",
        )
        .unwrap(),
        Felt::from_dec_str(
            "393083682526902362021532263518007213667960307630114380176309035415220154175",
        )
        .unwrap(),
        Felt::from_dec_str(
            "228421633383768634897088966132917335392548875057781107304223464921146078457",
        )
        .unwrap(),
        Felt::from_dec_str(
            "924492636261377530436326039939616684788498986913092833064848468110206034446",
        )
        .unwrap(),
        Felt::from_dec_str(
            "931874247810412416165687814293757020658355126150232036748630683953363661570",
        )
        .unwrap(),
        Felt::from_dec_str(
            "3215569544372428665026066156374535625549998775193271484978178438432055593239",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1239648551389298716869809742099762563284007154271743710729395207873328754095",
        )
        .unwrap(),
        Felt::from_dec_str(
            "89314992381896491523088980471634454610386898184975529899102140430198625686",
        )
        .unwrap(),
        Felt::from_dec_str(
            "3187985426015104063712461074067156374723555157039404691369397066532053540037",
        )
        .unwrap(),
    ];
    let expected_next_queries = vec![
        FriLayerQuery {
            index: Felt::from(1),
            y_value: Felt::from_dec_str(
                "3517289172167603711982011298083512014029690033243282325518726512433806986800",
            )
            .unwrap(),
            x_inv_value: Felt::from_dec_str(
                "1116757482133226363159883558057382197314289333955735205211445722872102609428",
            )
            .unwrap(),
        },
        FriLayerQuery {
            index: Felt::from(3),
            y_value: Felt::from_dec_str(
                "2418454055988269773691328349827589783726829031083650466961892768926964470890",
            )
            .unwrap(),
            x_inv_value: Felt::from_dec_str(
                "3351334608504925878396027162037002984645156260554050897526626427695988305073",
            )
            .unwrap(),
        },
    ];

    run_compute_next_layer_test(
        queries,
        sibling_witness,
        coset_size,
        eval_point,
        expected_verify_indices,
        expected_verify_y_values,
        expected_next_queries,
    );
}

#[test]
fn test_next_layer16() {
    let queries = vec![
        FriLayerQuery {
            index: Felt::from(7),
            y_value: Felt::from_dec_str(
                "2970618271957846642342346764466826063381003093310142263411672089332794267936",
            )
            .unwrap(),
            x_inv_value: Felt::from_dec_str(
                "696367657482296552349796637489145592059647766092429860960056113503758236140",
            )
            .unwrap(),
        },
        FriLayerQuery {
            index: Felt::from(15),
            y_value: Felt::from_dec_str(
                "3617920250969127201709777488707208906453088850534187183520682603094272928673",
            )
            .unwrap(),
            x_inv_value: Felt::from_dec_str(
                "2310133886796103194691263229141735996023430718973315327189200842370523874475",
            )
            .unwrap(),
        },
        FriLayerQuery {
            index: Felt::from(20),
            y_value: Felt::from_dec_str(
                "2813510573038804010549182461227254081026726929362216457735799917295807557921",
            )
            .unwrap(),
            x_inv_value: Felt::from_dec_str(
                "3136912033480025440798126631316085964287892167044727093790915521436523583198",
            )
            .unwrap(),
        },
        FriLayerQuery {
            index: Felt::from(27),
            y_value: Felt::from_dec_str(
                "2277310758645048528057712942822918302585095765258964079440830614234208634453",
            )
            .unwrap(),
            x_inv_value: Felt::from_dec_str(
                "1870747205314942372902696401365132965517762966704042444182117300433681492666",
            )
            .unwrap(),
        },
    ];

    let sibling_witness = vec![
        Felt::from_dec_str(
            "180909023427284908087068954735501773245381309706885307634112035720217679110",
        )
        .unwrap(),
        Felt::from_dec_str(
            "3516180717011553731559375883606897325832921300415665629181132188828019272220",
        )
        .unwrap(),
        Felt::from_dec_str(
            "595855061896376529488642887156753604023810348347280472583907714516066828172",
        )
        .unwrap(),
        Felt::from_dec_str(
            "3191235363083292632152220281591154669252475323788852857089686800728410534498",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1813822547938332577999867038562586426023262706228694365068812860876915662240",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1680128694077508680748055866477369978665203502934465079629899601611040128864",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2254538912986936078053356255795224792937407530312557714459538221668304707673",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1146541571666156127290331652094519929859911664578484739484350628703870274239",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2676242402429982706240946875896463048455086361143114665112503727666130770278",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1535606142568082504153083317992489160493054337756138059888305406164806817367",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2327833743689290796289689852734820784384652608694315731354512839605195535594",
        )
        .unwrap(),
        Felt::from_dec_str(
            "704019990774920412035974507851267537536910401523746675504830418223883704486",
        )
        .unwrap(),
        Felt::from_dec_str(
            "819141808945878073660426874084155050645640426465236632642039400376831935514",
        )
        .unwrap(),
        Felt::from_dec_str(
            "916452161521954486046652431769043932403407207121236049985828805160129320470",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2783827343576601773044240366051443811775748296762833339085024659475074755650",
        )
        .unwrap(),
        Felt::from_dec_str(
            "711792960833666366752081806111336906962181709004718383889531208840309577986",
        )
        .unwrap(),
        Felt::from_dec_str(
            "380931428319285290225787627529471042566157302138735167253381618771362729861",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2856797988975711654276161369071199892749361894922820391361113510391706048304",
        )
        .unwrap(),
        Felt::from_dec_str(
            "3482933488692241523840784131023860208574894633803191769610254610727766442033",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1264899078355765869100614221263650222437429703885704857993771751181043986946",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1400484696922187703290919818865394595993978084275297840720974996479598720354",
        )
        .unwrap(),
        Felt::from_dec_str(
            "107842594321822547920522338615937431539676850957901523579287900994294462578",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1660688899193255147418136637041572395267277578278543414089972502504493135495",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2203031443746817593668322965414946570177702172611895846085976472340779485221",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2587809142332512931520259907768113553396624721896300372775193299798500202556",
        )
        .unwrap(),
        Felt::from_dec_str(
            "3589911342720590439499798061492779810823748230816082995862628665524054571607",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2213453230887497720575673069515446618971622145035522840843819369864164611166",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2117829586952619081545500943334082335605420332616462445739700555609069867362",
        )
        .unwrap(),
    ];

    let coset_size = Felt::from(16);
    let eval_point = Felt::from_dec_str(
        "2131161197067646908062806238455827830521701314225315670510309581734422726318",
    )
    .unwrap();

    let expected_verify_indices = vec![Felt::from(0), Felt::from(1)];
    let expected_verify_y_values = vec![
        Felt::from_dec_str(
            "180909023427284908087068954735501773245381309706885307634112035720217679110",
        )
        .unwrap(),
        Felt::from_dec_str(
            "3516180717011553731559375883606897325832921300415665629181132188828019272220",
        )
        .unwrap(),
        Felt::from_dec_str(
            "595855061896376529488642887156753604023810348347280472583907714516066828172",
        )
        .unwrap(),
        Felt::from_dec_str(
            "3191235363083292632152220281591154669252475323788852857089686800728410534498",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1813822547938332577999867038562586426023262706228694365068812860876915662240",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1680128694077508680748055866477369978665203502934465079629899601611040128864",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2254538912986936078053356255795224792937407530312557714459538221668304707673",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2970618271957846642342346764466826063381003093310142263411672089332794267936",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1146541571666156127290331652094519929859911664578484739484350628703870274239",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2676242402429982706240946875896463048455086361143114665112503727666130770278",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1535606142568082504153083317992489160493054337756138059888305406164806817367",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2327833743689290796289689852734820784384652608694315731354512839605195535594",
        )
        .unwrap(),
        Felt::from_dec_str(
            "704019990774920412035974507851267537536910401523746675504830418223883704486",
        )
        .unwrap(),
        Felt::from_dec_str(
            "819141808945878073660426874084155050645640426465236632642039400376831935514",
        )
        .unwrap(),
        Felt::from_dec_str(
            "916452161521954486046652431769043932403407207121236049985828805160129320470",
        )
        .unwrap(),
        Felt::from_dec_str(
            "3617920250969127201709777488707208906453088850534187183520682603094272928673",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2783827343576601773044240366051443811775748296762833339085024659475074755650",
        )
        .unwrap(),
        Felt::from_dec_str(
            "711792960833666366752081806111336906962181709004718383889531208840309577986",
        )
        .unwrap(),
        Felt::from_dec_str(
            "380931428319285290225787627529471042566157302138735167253381618771362729861",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2856797988975711654276161369071199892749361894922820391361113510391706048304",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2813510573038804010549182461227254081026726929362216457735799917295807557921",
        )
        .unwrap(),
        Felt::from_dec_str(
            "3482933488692241523840784131023860208574894633803191769610254610727766442033",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1264899078355765869100614221263650222437429703885704857993771751181043986946",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1400484696922187703290919818865394595993978084275297840720974996479598720354",
        )
        .unwrap(),
        Felt::from_dec_str(
            "107842594321822547920522338615937431539676850957901523579287900994294462578",
        )
        .unwrap(),
        Felt::from_dec_str(
            "1660688899193255147418136637041572395267277578278543414089972502504493135495",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2203031443746817593668322965414946570177702172611895846085976472340779485221",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2277310758645048528057712942822918302585095765258964079440830614234208634453",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2587809142332512931520259907768113553396624721896300372775193299798500202556",
        )
        .unwrap(),
        Felt::from_dec_str(
            "3589911342720590439499798061492779810823748230816082995862628665524054571607",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2213453230887497720575673069515446618971622145035522840843819369864164611166",
        )
        .unwrap(),
        Felt::from_dec_str(
            "2117829586952619081545500943334082335605420332616462445739700555609069867362",
        )
        .unwrap(),
    ];
    let expected_next_queries = vec![
        FriLayerQuery {
            index: Felt::from(0),
            y_value: Felt::from_dec_str(
                "1673087752917740433456711006621617791163224131325431924340953404769449739057",
            )
            .unwrap(),
            x_inv_value: Felt::from_dec_str(
                "3563079260215329529465469507049655514938826357303290347407524859824127636176",
            )
            .unwrap(),
        },
        FriLayerQuery {
            index: Felt::from(1),
            y_value: Felt::from_dec_str(
                "411308955591734803942114330163581819505644029281528997154872777951395903682",
            )
            .unwrap(),
            x_inv_value: Felt::from_dec_str(
                "55423528450801684231853276045414590684280858028306352565567196311744384305",
            )
            .unwrap(),
        },
    ];

    run_compute_next_layer_test(
        queries,
        sibling_witness,
        coset_size,
        eval_point,
        expected_verify_indices,
        expected_verify_y_values,
        expected_next_queries,
    );
}

#[test]
fn test_two_compute_next_layer_tasks() {
    println!("=== Testing two consecutive ComputeNextLayer tasks ===");
    
    let mut stack = BidirectionalStackAccount::default();
    
    // First layer
    let first_layer_queries = vec![
        FriLayerQuery {
            index: Felt::from(19),
            y_value: Felt::from_dec_str("3009640132648008425771663319959959262486220333099664427328058765768842449250").unwrap(),
            x_inv_value: Felt::from_dec_str("54553547539894122403827046258314152559223416585866015792288871627619859008").unwrap(),
        },
    ];
    
    let first_layer_sibling_witness = vec![
        Felt::from_dec_str("1123008634785466227765787920403783137942925653310144335875674694591276473192").unwrap(),
    ];
    
    let coset_size = Felt::from(2);
    let eval_point = Felt::from_dec_str("2443479172752326919986485065418726216145528469562787664528210275186695390471").unwrap();
    
    let fri_verify_data = setup_fri_verify_data(
        first_layer_queries.clone(),
        coset_size,
        eval_point,
        first_layer_sibling_witness.clone(),
    );
    
    stack.store_in_cache(&fri_verify_data);
    
    // Run first ComputeNextLayer
    let first_task = ComputeNextLayer::new();
    stack.push_task(first_task);
    
    while !stack.is_empty_back() {
        stack.execute();
    }
    
    // Get first layer results
    let fri_verify_data_after_first = stack.borrow_from_cache::<FriVerifyData>();
    let first_layer_next_queries = {
        let mut queries = Vec::new();
        for i in 0..fri_verify_data_after_first.next_queries.len() {
            if let Some(query) = fri_verify_data_after_first.next_queries.get(i) {
                queries.push(*query);
            }
        }
        queries
    };
    
    let first_layer_y_values = {
        let mut values = Vec::new();
        for i in 0..fri_verify_data_after_first.working_y_values.len() {
            if let Some(value) = fri_verify_data_after_first.working_y_values.get(i) {
                values.push(*value);
            }
        }
        values
    };
    
    println!("First layer y_values: {:?}", first_layer_y_values);
    
    // Second layer - simulate FriVerifyLayers transition
    let mut fri_verify_data_for_second = stack.borrow_from_cache_mut::<FriVerifyData>();
    
    // Update working_queries with next_queries from first layer
    for query in &first_layer_next_queries {
        fri_verify_data_for_second.working_queries.push(*query);
    }
    
    // Add DIFFERENT sibling witness for second layer
    let second_layer_sibling_witness = vec![
        Felt::from_dec_str("9999999999999999999999999999999999999999999999999999999999999999").unwrap(),
    ];

    for witness in &second_layer_sibling_witness {
        fri_verify_data_for_second.sibling_witness.push(*witness);
    }
    
    // Run second ComputeNextLayer
    let second_task = ComputeNextLayer::new();
    stack.push_task(second_task);
    
    while !stack.is_empty_back() {
        stack.execute();
    }
    
    // Get second layer results
    let fri_verify_data_after_second = stack.borrow_from_cache::<FriVerifyData>();
    let second_layer_y_values = {
        let mut values = Vec::new();
        for i in 0..fri_verify_data_after_second.working_y_values.len() {
            if let Some(value) = fri_verify_data_after_second.working_y_values.get(i) {
                values.push(*value);
            }
        }
        values
    };
    
    println!("Second layer y_values: {:?}", second_layer_y_values);
    
    // Verification
    assert!(first_layer_y_values.len() > 0, "First layer should produce y_values");
    assert!(second_layer_y_values.len() > 0, "Second layer should produce y_values");
    
    // CRITICAL: Verify second layer uses its own sibling witness
    let found_second_witness = second_layer_y_values.contains(&second_layer_sibling_witness[0]);
    assert!(found_second_witness, "Second layer should use its own sibling witness");
    
    // Verify layers produce different results (no data leakage)
    let values_different = first_layer_y_values[0] != second_layer_y_values[0];
    assert!(values_different, "Layers should produce different y_values");
    
    println!("✅ Two consecutive ComputeNextLayer tasks test passed!");
}

