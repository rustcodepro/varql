use crate::combine::combinevariant;
use crate::pad::paddna;
use crate::pad::paddnagc;
use async_graphql::Object;
use smartcore::linalg::basic::matrix::DenseMatrix;
use smartcore::linear::logistic_regression::LogisticRegression;
use smartcore::metrics::accuracy;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

/*
 Gaurav Sablok
 codeprog@icloud.com
*/

#[derive(Debug, Default)]
pub struct Query;

#[Object]
impl Query {
    pub async fn machinelearning(
        &self,
        pathvariant: String,
        pathsam: String,
        quality: String,
        filtervalue: String,
        fastapredict: String,
    ) -> std::io::Result<String> {
        let combinedvalue = combinevariant(pathvariant, pathsam).await.unwrap();
        let value = quality.parse::<usize>().unwrap();
        let mut classificationlabel: Vec<i32> = Vec::new();
        let mut valuevec: Vec<Vec<f32>> = Vec::new();
        /*
         * collecting all the mapped read specific to that variant
         * and then concatenating them
         * into a single sequence before converting into a tensor.
         */
        for i in combinedvalue.iter() {
            if i.quality.parse::<usize>().unwrap() > value && i.filter == filtervalue {
                let valuestring = i.seqvec.concat().to_string();
                let inputstring: Vec<char> = valuestring.chars().collect::<Vec<_>>();
                let mut vecinput: Vec<Vec<f32>> = Vec::new();
                for i in inputstring.iter() {
                    match i {
                        'A' => vecinput.push(vec![1.0, 0.0, 0.0, 0.0]),
                        'T' => vecinput.push(vec![0.0, 1.0, 0.0, 0.0]),
                        'G' => vecinput.push(vec![0.0, 0.0, 1.0, 0.0]),
                        'C' => vecinput.push(vec![0.0, 0.0, 0.0, 1.0]),
                        'N' => vecinput.push(vec![0.0, 0.0, 0.0, 0.0]),
                        _ => continue,
                    }
                }
                let gccontent = paddnagc(valuestring).await.unwrap();
                let qualityvalue = i.quality.parse::<usize>().unwrap();
                vecinput.push(vec![gccontent, qualityvalue as f32, 0.0, 0.0]);
                classificationlabel.push(0);
                valuevec.push(vecinput.iter().flatten().cloned().collect::<Vec<f32>>());
            }
            if i.quality.parse::<usize>().unwrap() < value && i.filter == filtervalue {
                let valuestring = i.seqvec.concat().to_string();
                let inputstring: Vec<char> = valuestring.chars().collect::<Vec<_>>();
                let mut vecinput: Vec<Vec<f32>> = Vec::new();
                for i in inputstring.iter() {
                    match i {
                        'A' => vecinput.push(vec![1.0, 0.0, 0.0, 0.0]),
                        'T' => vecinput.push(vec![0.0, 1.0, 0.0, 0.0]),
                        'G' => vecinput.push(vec![0.0, 0.0, 1.0, 0.0]),
                        'C' => vecinput.push(vec![0.0, 0.0, 0.0, 1.0]),
                        'N' => vecinput.push(vec![0.0, 0.0, 0.0, 0.0]),
                        _ => continue,
                    }
                }
                let gccontent = paddnagc(valuestring).await.unwrap();
                let qualityvalue = i.quality.parse::<usize>().unwrap();
                vecinput.push(vec![gccontent, qualityvalue as f32, 0.0, 0.0]);
                classificationlabel.push(1);
                valuevec.push(vecinput.iter().flatten().cloned().collect::<Vec<f32>>());
            }
        }

        let mut predict: Vec<String> = Vec::new();
        let predictopen = File::open(fastapredict).expect("file not present");
        let predictread = BufReader::new(predictopen);
        for i in predictread.lines() {
            let line = i.expect("line not present");
            predict.push(line);
        }

        let predictvec = paddna(predict).await.unwrap();
        let predictdensematrix = DenseMatrix::from_2d_vec(&predictvec).unwrap();

        let densematix: DenseMatrix<f32> = DenseMatrix::from_2d_vec(&valuevec).unwrap();
        let logistic_regression =
            LogisticRegression::fit(&densematix, &classificationlabel, Default::default()).unwrap();
        let modelfit = logistic_regression.predict(&predictdensematrix).unwrap();
        let accuracymodel = accuracy(&classificationlabel, &modelfit);
        println!("The accuracy of the predicted model is {:?}", accuracymodel);
        Ok("The machine learning results are as follows".to_string())
    }
}
